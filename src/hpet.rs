use crate::sdt::SdtHeader;
use crate::{AcpiError, GenericAddress, PhysicalMapping};

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Hpet {
    header: SdtHeader,

    pub event_timer_block_id: u32,
    pub base_address: GenericAddress,
    pub hpet_number: u8,
    pub clock_tick_unit: u16,
    pub page_protection_oem: u8,
}

pub fn parse_hpet(mapping: &PhysicalMapping<Hpet>) -> Result<(), AcpiError> {
    (*mapping).header.validate(b"HPET")?;

    Ok(())
}
