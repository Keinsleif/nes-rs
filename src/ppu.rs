mod registers;

use self::registers::{
    AddrRegister, ControlRegister, MaskRegister, ScrollRegister, StatusRegister,
};
use crate::cartridge::Mirroring;

pub enum TileId {
    Normal { id: u8 },
    Large {
        bank: u16,
        id: u8,
    }
}

impl TileId {
    pub fn new(is_large: bool, src: u8) -> Self {
        if !is_large {
            TileId::Normal { id: src }
        } else {
            TileId::Large { bank: (if (src & 0x01) == 0x01 {
                0x1000
            } else {
                0x0000
            }), id: src & 0xfe }
        }
    }
}

pub struct SpriteAttr {
    pub palette: u8,
    pub priority: u8,
    pub is_flip_horizonal: bool,
    pub is_flip_vertical: bool,
}

impl SpriteAttr {
    pub fn new(src: u8) -> Self {
        SpriteAttr {
            palette: src & 0b0000_0011,
            priority: (src >> 5) & 0b0000_0001,
            is_flip_horizonal: src & 0b0100_0000 != 0x00,
            is_flip_vertical: src & 0b1000_0000 != 0x00,
        }
    }
}

pub struct Sprite {
    pub y: u8,
    pub tile_id: TileId,
    pub attr: SpriteAttr,
    pub x: u8,
}

impl Sprite {
    pub fn new(is_large: bool, oam: [u8; 4]) -> Self {
        Sprite {
            y: oam[0],
            tile_id: TileId::new(is_large, oam[1]),
            attr: SpriteAttr::new(oam[2]),
            x: oam[3],
        }
    }
}

enum LineStatus {
    Visible,
    PostRender,
    VerticalBlanking(bool),
    PreRender,
}
impl LineStatus {
    fn from(line: u16) -> LineStatus {
        if line < 240 {
            LineStatus::Visible
        } else if line == 240 {
            LineStatus::PostRender
        } else if line < 261 {
            LineStatus::VerticalBlanking(line == 241)
        } else if line == 261 {
            LineStatus::PreRender
        } else {
            panic!("invalid line status")
        }
    }
}

pub struct NesPPU {
    pub chr_rom: Vec<u8>,
    pub is_chr_ram: bool,
    pub mirroring: Mirroring,
    pub palette_table: [u8; 32],

    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub scroll: ScrollRegister,
    pub addr: AddrRegister,
    pub vram: [u8; 2048],

    internal_data_buf: u8,

    scanline: u16,
    cycles: usize,
    pub nmi_interrupt: Option<u8>,
}

pub trait PPU {
    fn write_to_ctrl(&mut self, value: u8);
    fn write_to_mask(&mut self, value: u8);
    fn read_status(&mut self) -> u8;
    fn write_to_oam_addr(&mut self, value: u8);
    fn write_to_oam_data(&mut self, value: u8);
    fn read_oam_data(&self) -> u8;
    fn write_to_scroll(&mut self, value: u8);
    fn write_to_ppu_addr(&mut self, value: u8);
    fn write_to_data(&mut self, value: u8);
    fn read_data(&mut self) -> u8;
    fn write_oam_dma(&mut self, value: &[u8; 256]);
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, is_chr_ram: bool, mirroring: Mirroring) -> Self {
        NesPPU {
            chr_rom,
            is_chr_ram,
            mirroring,
            palette_table: [0; 32],
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            oam_addr: 0,
            oam_data: [0; 64 * 4],
            scroll: ScrollRegister::new(),
            addr: AddrRegister::new(),
            vram: [0; 2048],
            internal_data_buf: 0,

            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]

    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        let y = self.oam_data[0] as usize;
        let x = self.oam_data[3] as usize;
        (y == self.scanline as usize) && x <= cycle && self.mask.show_sprites()
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
        if self.cycles >= 341 {

            if self.is_sprite_0_hit(self.cycles as usize) {
                self.status.set_sprite_zero_hit(true)
            }
            
            self.cycles = self.cycles - 341;
            self.scanline += 1;

            match LineStatus::from(self.scanline) {
                LineStatus::Visible => {}
                LineStatus::PostRender => {}
                LineStatus::VerticalBlanking(is_first) => {
                    if is_first {
                        self.status.set_vblank_status(true);
                        self.status.set_sprite_zero_hit(false);
                        if self.ctrl.generate_vblank_nmi() {
                            self.nmi_interrupt = Some(1);
                        }
                    }
                }
                LineStatus::PreRender => {
                    self.nmi_interrupt = None;
                    self.status.set_sprite_zero_hit(false);
                    self.status.reset_vblank_status();
                }
            }
            self.scanline = (self.scanline + 1) % 262
        }
    }

    fn fetch_sprite(&mut self) {
        if !self.mask.show_sprites() {
            return;
        }
        let sprite_height = u16::from(self.ctrl.sprite_size());
        let mut tmp_idx = 0;
        for sprite_idx in 0..64 {
            let oam_addr = sprite_idx << 2;
            let sprite_y = u16::from(self.oam_data[oam_addr]);
            let sprite_y_end = sprite_y + sprite_height;
            
            if (sprite_y < self.scanline) && (sprite_y_end >= self.scanline) {
                
            }
        }
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }
}

impl PPU for NesPPU {
    fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.ctrl.generate_vblank_nmi();
        self.ctrl.update(value);
        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    fn read_status(&mut self) -> u8 {
        let data = self.status.snapshot();
        self.status.reset_vblank_status();
        self.addr.reset_latch();
        self.scroll.reset_latch();
        data
    }

    fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    fn write_to_scroll(&mut self, value: u8) {
        self.scroll.write(value);
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            0..=0x1fff => {
                if self.is_chr_ram {
                    self.chr_rom[addr as usize] = value;
                } else {
                    println!("Attempt to write to chr rom space 0x{:<04x}", addr)
                }
            },
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3eff => {
                let addr_mirror = addr - 0x1000;
                self.vram[self.mirror_vram_addr(addr_mirror) as usize] = value;
            }
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[((add_mirror - 0x3f00) as usize)%32] = value;
            }
            0x3f00..=0x3fff => {
                self.palette_table[((addr - 0x3f00) as usize)%32] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }

            0x3000..=0x3eff => {
                let addr_mirror = addr - 0x1000;
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr_mirror) as usize];
                result
                // panic(
                // "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                // addr
                // )
            }

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[((add_mirror - 0x3f00) as usize)%32]
            }

            0x3f00..=0x3fff => self.palette_table[((addr - 0x3f00) as usize)%32],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn write_oam_dma(&mut self, value: &[u8; 256]) {
        for x in value.iter() {
            self.oam_data[self.oam_addr as usize] = *x;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        }
    }
}
