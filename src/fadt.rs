use crate::aml::{parse_aml_table, AmlTable};
use crate::sdt;
use crate::sdt::SdtHeader;
use crate::{Acpi, AcpiError, AcpiHandler, GenericAddress, PhysicalMapping};

/// Represents the Fixed ACPI Description Table (FADT). This table contains various fixed hardware
/// details, such as the addresses of the hardware register blocks. It also contains a pointer to
/// the Differentiated Definition Block (DSDT).
///
/// In cases where the FADT contains both a 32-bit and 64-bit field for the same address, we should
/// always prefer the 64-bit one. Only if it's zero or the CPU will not allow us to access that
/// address should the 32-bit one be used.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Fadt {
    header: SdtHeader,

    pub firmware_ctrl: u32,
    pub dsdt_address: u32,

    // used in acpi 1.0; compatibility only, should be zero
    reserved: u8,

    pub preferred_pm_profile: u8,
    pub sci_interrupt: u16,
    pub smi_cmd_port: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_control: u8,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm2_control_block: u32,
    pub pm_timer_block: u32,
    pub gpe0_block: u32,
    pub gpe1_block: u32,
    pub pm1_event_length: u8,
    pub pm1_control_length: u8,
    pub pm2_control_length: u8,
    pub pm_timer_length: u8,
    pub gpe0_block_length: u8,
    pub gpe1_block_length: u8,
    pub gpe1_base: u8,
    pub c_state_control: u8,
    pub worst_c2_latency: u16,
    pub worst_c3_latency: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alarm: u8,
    pub month_alarm: u8,
    pub century: u8,
    pub iapc_boot_arch: u16,
    reserved2: u8, // must be 0
    pub flags: u32,
    pub reset_reg: GenericAddress,
    pub reset_value: u8,
    pub arm_boot_arch: u16,
    pub fadt_minor_version: u8,
    pub x_firmware_control: u64,
    pub x_dsdt_address: u64,
    pub x_pm1a_event_block: GenericAddress,
    pub x_pm1b_event_block: GenericAddress,
    pub x_pm1a_control_block: GenericAddress,
    pub x_pm1b_control_block: GenericAddress,
    pub x_pm2_control_block: GenericAddress,
    pub x_pm_timer_block: GenericAddress,
    pub x_gpe0_block: GenericAddress,
    pub x_gpe1_block: GenericAddress,
    pub sleep_control_reg: GenericAddress,
    pub sleep_status_reg: GenericAddress,
    pub hypervisor_vendor_id: u64,
}

pub(crate) fn parse_fadt<H>(
    acpi: &mut Acpi,
    handler: &mut H,
    mapping: &PhysicalMapping<Fadt>,
) -> Result<(), AcpiError>
where
    H: AcpiHandler,
{
    let fadt = &*mapping;
    fadt.header.validate(b"FACP")?;

    // TODO more generic typesafe way of accessing the x_ fields
    let dsdt_physical_address: usize = if fadt.header.revision() > 1 && fadt.x_dsdt_address != 0 {
        fadt.x_dsdt_address as usize
    } else {
        fadt.dsdt_address as usize
    };

    // Parse the DSDT
    let dsdt_header = sdt::peek_at_sdt_header(handler, dsdt_physical_address);
    let dsdt_mapping = handler
        .map_physical_region::<AmlTable>(dsdt_physical_address, dsdt_header.length() as usize);
    if let Err(error) = parse_aml_table(acpi, handler, &dsdt_mapping, b"DSDT") {
        error!("Failed to parse DSDT: {:?}. At this stage, this is expected, but should be fatal in the future", error);
    }
    handler.unmap_physical_region(dsdt_mapping);
    acpi.fadt = Some(**mapping);

    Ok(())
}
