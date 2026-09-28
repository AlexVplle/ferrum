#[path = "../../shared/paging/page_table.rs"]
pub mod page_table;
#[path = "../../shared/paging/page_table_entry.rs"]
pub mod page_table_entry;
#[path = "../../shared/paging/page_table_entry_flags.rs"]
pub mod page_table_entry_flags;
#[path = "../../shared/paging/tlb.rs"]
pub mod tlb;

mod constants;
pub mod early_paging;
pub mod satp;
pub mod setup;
