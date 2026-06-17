use crate::{
    models::{
        model_filters::LastModifyFilter,
        model_task_transfer::{IncrementMode, TransferType},
    },
    tasks::task_id_generator,
};

pub struct TaskDefaultParameters {}

impl TaskDefaultParameters {
    pub fn id_default() -> String {
        task_id_generator().to_string()
    }

    pub fn name_default() -> String {
        "default_name".to_string()
    }

    pub fn objects_compare_batch_default() -> i32 {
        256
    }

    pub fn objects_transfer_batch_default() -> i32 {
        64
    }

    pub fn objects_list_batch_default() -> i32 {
        512
    }

    pub fn objects_list_files_max_line_default() -> usize {
        1000000
    }

    pub fn task_parallelism_default() -> usize {
        num_cpus::get() * 4
    }

    pub fn start_from_checkpoint_default() -> bool {
        false
    }

    pub fn exprirs_diff_scope_default() -> i64 {
        10
    }

    pub fn target_exists_skip_default() -> bool {
        false
    }
    pub fn large_file_size_default() -> usize {
        // 64M
        1048576 * 64
    }
    pub fn multi_part_chunk_size_default() -> usize {
        // 8M
        1048576 * 8
    }

    pub fn multi_part_chunks_per_batch_default() -> usize {
        16
    }

    pub fn multi_part_parallelism_default() -> usize {
        num_cpus::get() * 2
    }

    pub fn multi_part_max_parallelism_default() -> usize {
        num_cpus::get() * 3
    }

    pub fn meta_dir_default() -> String {
        "/tmp/meta_dir".to_string()
    }

    pub fn filter_default() -> Option<Vec<String>> {
        None
    }

    pub fn transfer_type_default() -> TransferType {
        TransferType::Stock
    }

    pub fn last_modify_filter_default() -> Option<LastModifyFilter> {
        None
    }
    pub fn objects_list_files_default() -> Option<Vec<String>> {
        None
    }

    pub fn increment_mode_default() -> IncrementMode {
        IncrementMode::default()
    }

    pub fn preserve_prefix_default() -> bool {
        true
    }
}
