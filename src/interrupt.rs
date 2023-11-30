#[derive(PartialEq, Eq)]
pub enum InterruptType {
    NMI,
}

#[derive(PartialEq, Eq)]
pub(super) struct Interrupt {
    pub(super) itype: InterruptType,
    pub(super) vector_addr: u16,
    pub(super) b_flag_mask: u8,
    pub(super) cpu_cycles: u8,
}
pub(super) const NMI: Interrupt = Interrupt {
    itype: InterruptType::NMI,
    vector_addr: 0xfffA,
    b_flag_mask: 0b00100000,
    cpu_cycles: 2,
};
