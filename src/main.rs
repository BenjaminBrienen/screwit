// clippy
//  cargo (default=allow)
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::negative_feature_names)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::wildcard_dependencies)]
//  complexity (default=warn)
#![warn(clippy::bind_instead_of_map)]
#![warn(clippy::bool_comparison)]
#![warn(clippy::borrow_deref_ref)]
#![warn(clippy::borrowed_box)]
#![warn(clippy::bytes_count_to_len)]
#![warn(clippy::char_lit_as_u8)]
#![warn(clippy::clone_on_copy)]
#![warn(clippy::crosspointer_transmute)]
#![warn(clippy::deprecated_cfg_attr)]
#![warn(clippy::deref_addrof)]
#![warn(clippy::derivable_impls)]
#![warn(clippy::diverging_sub_expression)]
#![warn(clippy::double_comparisons)]
#![warn(clippy::double_parens)]
#![warn(clippy::duration_subsec)]
#![warn(clippy::explicit_auto_deref)]
#![warn(clippy::explicit_counter_loop)]
#![warn(clippy::explicit_write)]
#![warn(clippy::extra_unused_lifetimes)]
#![warn(clippy::filter_map_identity)]
#![warn(clippy::filter_next)]
#![warn(clippy::flat_map_identity)]
#![warn(clippy::get_last_with_len)]
#![warn(clippy::identity_op)]
#![warn(clippy::inspect_for_each)]
#![warn(clippy::int_plus_one)]
#![warn(clippy::iter_count)]
#![warn(clippy::iter_kv_map)]
#![warn(clippy::manual_clamp)]
#![warn(clippy::manual_filter)]
#![warn(clippy::manual_filter_map)]
#![warn(clippy::manual_find)]
#![warn(clippy::manual_find_map)]
#![warn(clippy::manual_flatten)]
#![warn(clippy::manual_rem_euclid)]
#![warn(clippy::manual_split_once)]
#![warn(clippy::manual_strip)]
#![warn(clippy::manual_swap)]
#![warn(clippy::manual_unwrap_or)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_identity)]
#![warn(clippy::match_as_ref)]
#![warn(clippy::match_single_binding)]
#![warn(clippy::needless_arbitrary_self_type)]
#![warn(clippy::needless_bool)]
#![warn(clippy::needless_borrowed_reference)]
#![warn(clippy::needless_lifetimes)]
#![warn(clippy::needless_match)]
#![warn(clippy::needless_option_as_deref)]
#![warn(clippy::needless_option_take)]
#![warn(clippy::needless_question_mark)]
#![warn(clippy::needless_splitn)]
#![warn(clippy::needless_update)]
#![warn(clippy::neg_cmp_op_on_partial_ord)]
#![warn(clippy::no_effect)]
#![warn(clippy::nonminimal_bool)]
#![warn(clippy::only_used_in_recursion)]
#![warn(clippy::option_as_ref_deref)]
#![warn(clippy::option_filter_map)]
#![warn(clippy::option_map_unit_fn)]
#![warn(clippy::or_then_unwrap)]
#![warn(clippy::overflow_check_conditional)]
#![warn(clippy::partialeq_ne_impl)]
#![warn(clippy::precedence)]
#![warn(clippy::ptr_offset_with_cast)]
#![warn(clippy::range_zip_with_len)]
#![warn(clippy::redundant_closure_call)]
#![warn(clippy::redundant_slicing)]
#![warn(clippy::repeat_once)]
#![warn(clippy::result_map_unit_fn)]
#![warn(clippy::search_is_some)]
#![warn(clippy::seek_from_current)]
#![warn(clippy::seek_to_start_instead_of_rewind)]
#![warn(clippy::short_circuit_statement)]
#![warn(clippy::single_element_loop)]
#![warn(clippy::skip_while_next)]
#![warn(clippy::string_from_utf8_as_bytes)]
#![warn(clippy::strlen_on_c_strings)]
#![warn(clippy::temporary_assignment)]
#![warn(clippy::too_many_arguments)]
#![warn(clippy::transmute_bytes_to_str)]
#![warn(clippy::transmute_float_to_int)]
#![warn(clippy::transmute_int_to_bool)]
#![warn(clippy::transmute_int_to_char)]
#![warn(clippy::transmute_int_to_float)]
#![warn(clippy::transmute_num_to_bytes)]
#![warn(clippy::transmute_ptr_to_ref)]
#![warn(clippy::transmutes_expressible_as_ptr_casts)]
#![warn(clippy::type_complexity)]
#![warn(clippy::unit_arg)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::unnecessary_filter_map)]
#![warn(clippy::unnecessary_find_map)]
#![warn(clippy::unnecessary_operation)]
#![warn(clippy::unnecessary_sort_by)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::unneeded_wildcard_pattern)]
#![warn(clippy::unused_format_specs)]
#![warn(clippy::useless_asref)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::useless_format)]
#![warn(clippy::useless_transmute)]
#![warn(clippy::vec_box)]
#![warn(clippy::while_let_loop)]
#![warn(clippy::wildcard_in_or_patterns)]
#![warn(clippy::zero_divided_by_zero)]
#![warn(clippy::zero_prefixed_literal)]
//  correctness (default=deny)
#![deny(clippy::absurd_extreme_comparisons)]
#![deny(clippy::almost_swapped)]
#![deny(clippy::approx_constant)]
#![deny(clippy::async_yields_async)]
#![deny(clippy::bad_bit_mask)]
#![deny(clippy::cast_ref_to_mut)]
#![deny(clippy::cast_slice_different_sizes)]
#![deny(clippy::clone_double_ref)]
#![deny(clippy::cmp_nan)]
#![deny(clippy::deprecated_semver)]
#![deny(clippy::derive_hash_xor_eq)]
#![deny(clippy::derive_ord_xor_partial_ord)]
#![deny(clippy::drop_copy)]
#![deny(clippy::drop_ref)]
#![deny(clippy::enum_clike_unportable_variant)]
#![deny(clippy::eq_op)]
#![deny(clippy::erasing_op)]
#![deny(clippy::fn_address_comparisons)]
#![deny(clippy::forget_copy)]
#![deny(clippy::forget_ref)]
#![deny(clippy::if_let_mutex)]
#![deny(clippy::if_same_then_else)]
#![deny(clippy::ifs_same_cond)]
#![deny(clippy::ineffective_bit_mask)]
#![deny(clippy::infinite_iter)]
#![deny(clippy::inherent_to_string_shadow_display)]
#![deny(clippy::inline_fn_without_body)]
#![deny(clippy::invalid_null_ptr_usage)]
#![deny(clippy::invalid_regex)]
#![deny(clippy::invalid_utf8_in_unchecked)]
#![deny(clippy::invisible_characters)]
#![deny(clippy::iter_next_loop)]
#![deny(clippy::iterator_step_by_zero)]
#![deny(clippy::let_underscore_lock)]
#![deny(clippy::match_str_case_mismatch)]
#![deny(clippy::mem_replace_with_uninit)]
#![deny(clippy::min_max)]
#![deny(clippy::mismatched_target_os)]
#![deny(clippy::mistyped_literal_suffixes)]
#![deny(clippy::modulo_one)]
#![deny(clippy::mut_from_ref)]
#![deny(clippy::never_loop)]
#![deny(clippy::non_octal_unix_permissions)]
#![deny(clippy::nonsensical_open_options)]
#![deny(clippy::not_unsafe_ptr_arg_deref)]
#![deny(clippy::option_env_unwrap)]
#![deny(clippy::out_of_bounds_indexing)]
#![deny(clippy::overly_complex_bool_expr)]
#![deny(clippy::panicking_unwrap)]
#![deny(clippy::possible_missing_comma)]
#![deny(clippy::read_zero_byte_vec)]
#![deny(clippy::recursive_format_impl)]
#![deny(clippy::reversed_empty_ranges)]
#![deny(clippy::self_assignment)]
#![deny(clippy::serde_api_misuse)]
#![deny(clippy::size_of_in_element_count)]
#![deny(clippy::suspicious_splitn)]
#![deny(clippy::transmuting_null)]
#![deny(clippy::undropped_manually_drops)]
#![deny(clippy::uninit_assumed_init)]
#![deny(clippy::uninit_vec)]
#![deny(clippy::unit_cmp)]
#![deny(clippy::unit_hash)]
#![deny(clippy::unit_return_expecting_ord)]
#![deny(clippy::unsound_collection_transmute)]
#![deny(clippy::unused_io_amount)]
#![deny(clippy::useless_attribute)]
#![deny(clippy::vec_resize_to_zero)]
#![deny(clippy::vtable_address_comparisons)]
#![deny(clippy::while_immutable_condition)]
#![deny(clippy::wrong_transmute)]
#![deny(clippy::zst_offset)]
//  nursery (default=allow)
#![warn(clippy::as_ptr_cast_mut)]
#![warn(clippy::branches_sharing_code)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::debug_assert_with_mut_call)]
#![warn(clippy::derive_partial_eq_without_eq)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::future_not_send)]
#![warn(clippy::imprecise_flops)]
#![warn(clippy::iter_on_empty_collections)]
#![warn(clippy::iter_on_single_items)]
#![warn(clippy::iter_with_drain)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::needless_collect)]
#![warn(clippy::non_send_fields_in_send_ty)]
#![warn(clippy::nonstandard_macro_braces)]
#![warn(clippy::option_if_let_else)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::path_buf_push_overwrite)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::significant_drop_in_scrutinee)]
#![warn(clippy::string_lit_as_bytes)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::suspicious_operation_groupings)]
#![warn(clippy::trailing_empty_array)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::transmute_undefined_repr)]
#![warn(clippy::trivial_regex)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unused_peekable)]
#![warn(clippy::unused_rounding)]
#![warn(clippy::use_self)]
#![warn(clippy::useless_let_if_seq)]
//  pedantic (default=allow)
#![warn(clippy::bool_to_int_with_if)]
#![warn(clippy::borrow_as_ptr)]
#![warn(clippy::case_sensitive_file_extension_comparisons)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_ptr_alignment)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::checked_conversions)]
#![warn(clippy::cloned_instead_of_copied)]
#![warn(clippy::copy_iterator)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::doc_link_with_quotes)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::empty_enum)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::expl_impl_clone_on_copy)]
#![warn(clippy::explicit_deref_methods)]
#![warn(clippy::explicit_into_iter_loop)]
#![warn(clippy::explicit_iter_loop)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::flat_map_option)]
#![warn(clippy::float_cmp)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::from_iter_instead_of_collect)]
#![warn(clippy::if_not_else)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::implicit_hasher)]
#![warn(clippy::inconsistent_struct_constructor)]
#![warn(clippy::index_refutable_slice)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::inline_always)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::iter_not_returning_iterator)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::linkedlist)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_instant_elapsed)]
#![warn(clippy::manual_let_else)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::manual_string_new)]
#![warn(clippy::many_single_char_names)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_on_vec_items)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::maybe_infinite_iter)]
#![warn(clippy::mismatching_type_param_order)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::mut_mut)]
#![warn(clippy::naive_bytecount)]
#![warn(clippy::needless_bitwise_bool)]
#![warn(clippy::needless_continue)]
#![warn(clippy::needless_for_each)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::no_effect_underscore_binding)]
#![warn(clippy::option_option)]
#![warn(clippy::ptr_as_ptr)]
#![warn(clippy::range_minus_one)]
#![warn(clippy::range_plus_one)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::redundant_else)]
#![warn(clippy::ref_binding_to_reference)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::same_functions_in_if_condition)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::similar_names)]
#![warn(clippy::single_match_else)]
#![warn(clippy::stable_sort_primitive)]
#![warn(clippy::string_add_assign)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::transmute_ptr_to_ptr)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::unicode_not_nfc)]
#![warn(clippy::unnecessary_join)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unsafe_derive_deserialize)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::verbose_bit_mask)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::zero_sized_map_values)]
//  performance (default=warn)
#![warn(clippy::box_collection)]
#![warn(clippy::box_default)]
#![warn(clippy::boxed_local)]
#![warn(clippy::cmp_owned)]
#![warn(clippy::collapsible_str_replace)]
#![warn(clippy::expect_fun_call)]
#![warn(clippy::extend_with_drain)]
#![warn(clippy::format_in_format_args)]
#![warn(clippy::iter_nth)]
#![warn(clippy::iter_overeager_cloned)]
#![warn(clippy::large_const_arrays)]
#![warn(clippy::large_enum_variant)]
#![warn(clippy::manual_memcpy)]
#![warn(clippy::manual_retain)]
#![warn(clippy::manual_str_repeat)]
#![warn(clippy::map_entry)]
#![warn(clippy::missing_spin_loop)]
#![warn(clippy::redundant_allocation)]
#![warn(clippy::redundant_clone)]
#![warn(clippy::result_large_err)]
#![warn(clippy::single_char_pattern)]
#![warn(clippy::slow_vector_initialization)]
#![warn(clippy::to_string_in_format_args)]
#![warn(clippy::unnecessary_to_owned)]
#![warn(clippy::useless_vec)]
#![warn(clippy::vec_init_then_push)]
//  restriction (default=allow)
#![allow(clippy::alloc_instead_of_core)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::as_conversions)]
#![allow(clippy::as_underscore)]
#![allow(clippy::assertions_on_result_states)]
#![allow(clippy::clone_on_ref_ptr)]
#![allow(clippy::create_dir)]
#![allow(clippy::dbg_macro)]
#![allow(clippy::decimal_literal_representation)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::default_union_representation)]
#![allow(clippy::deref_by_slicing)]
#![allow(clippy::disallowed_script_idents)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::empty_drop)]
#![allow(clippy::empty_structs_with_brackets)]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::exit)]
#![allow(clippy::expect_used)]
#![allow(clippy::filetype_is_file)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::float_cmp_const)]
#![allow(clippy::fn_to_numeric_cast_any)]
#![allow(clippy::format_push_string)]
#![allow(clippy::get_unwrap)]
#![allow(clippy::if_then_some_else_none)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::inline_asm_x86_att_syntax)]
#![allow(clippy::inline_asm_x86_intel_syntax)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::large_include_file)]
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::lossy_float_literal)]
#![allow(clippy::map_err_ignore)]
#![allow(clippy::mem_forget)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_enforced_import_renames)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::mixed_read_write_in_expression)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::modulo_arithmetic)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::panic)]
#![allow(clippy::panic_in_result_fn)]
#![allow(clippy::partial_pub_fields)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::print_stderr)]
#![allow(clippy::print_stdout)]
#![allow(clippy::pub_use)]
#![allow(clippy::rc_buffer)]
#![allow(clippy::rc_mutex)]
#![allow(clippy::rest_pat_in_fully_bound_structs)]
#![allow(clippy::same_name_method)]
#![allow(clippy::self_named_module_files)]
#![allow(clippy::separated_literal_suffix)]
#![allow(clippy::shadow_reuse)]
#![allow(clippy::shadow_same)]
#![allow(clippy::shadow_unrelated)]
#![allow(clippy::single_char_lifetime_names)]
#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::std_instead_of_core)]
#![allow(clippy::str_to_string)]
#![allow(clippy::string_add)]
#![allow(clippy::string_slice)]
#![allow(clippy::string_to_string)]
#![allow(clippy::suspicious_xor_used_as_pow)]
#![allow(clippy::todo)]
#![allow(clippy::try_err)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::unimplemented)]
#![allow(clippy::unnecessary_safety_comment)]
#![allow(clippy::unnecessary_safety_doc)]
#![allow(clippy::unnecessary_self_imports)]
#![allow(clippy::unneeded_field_pattern)]
#![allow(clippy::unreachable)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(clippy::unwrap_in_result)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::use_debug)]
#![allow(clippy::verbose_file_reads)]
#![allow(clippy::wildcard_enum_match_arm)]
//  style (default=warn)
#![warn(clippy::assertions_on_constants)]
#![warn(clippy::assign_op_pattern)]
#![warn(clippy::blocks_in_if_conditions)]
#![warn(clippy::bool_assert_comparison)]
#![warn(clippy::borrow_interior_mutable_const)]
#![warn(clippy::builtin_type_shadow)]
#![warn(clippy::bytes_nth)]
#![warn(clippy::chars_last_cmp)]
#![warn(clippy::chars_next_cmp)]
#![warn(clippy::cmp_null)]
#![warn(clippy::collapsible_else_if)]
#![warn(clippy::collapsible_if)]
#![warn(clippy::collapsible_match)]
#![warn(clippy::comparison_chain)]
#![warn(clippy::comparison_to_empty)]
#![warn(clippy::declare_interior_mutable_const)]
#![warn(clippy::default_instead_of_iter_empty)]
#![warn(clippy::disallowed_macros)]
#![warn(clippy::disallowed_methods)]
#![warn(clippy::disallowed_names)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::double_must_use)]
#![warn(clippy::double_neg)]
#![warn(clippy::duplicate_underscore_argument)]
#![warn(clippy::enum_variant_names)]
#![warn(clippy::err_expect)]
#![warn(clippy::excessive_precision)]
#![warn(clippy::field_reassign_with_default)]
#![warn(clippy::fn_to_numeric_cast)]
#![warn(clippy::fn_to_numeric_cast_with_truncation)]
#![warn(clippy::for_kv_map)]
#![warn(clippy::from_over_into)]
#![warn(clippy::from_str_radix_10)]
#![warn(clippy::get_first)]
#![warn(clippy::implicit_saturating_add)]
#![warn(clippy::implicit_saturating_sub)]
#![warn(clippy::inconsistent_digit_grouping)]
#![warn(clippy::infallible_destructuring_match)]
#![warn(clippy::inherent_to_string)]
#![warn(clippy::init_numbered_fields)]
#![warn(clippy::into_iter_on_ref)]
#![warn(clippy::is_digit_ascii_radix)]
#![warn(clippy::iter_cloned_collect)]
#![warn(clippy::iter_next_slice)]
#![warn(clippy::iter_nth_zero)]
#![warn(clippy::iter_skip_next)]
#![warn(clippy::just_underscores_and_digits)]
#![warn(clippy::len_without_is_empty)]
#![warn(clippy::len_zero)]
#![warn(clippy::let_and_return)]
#![warn(clippy::let_unit_value)]
#![warn(clippy::main_recursion)]
#![warn(clippy::manual_async_fn)]
#![warn(clippy::manual_bits)]
#![warn(clippy::manual_is_ascii_check)]
#![warn(clippy::manual_map)]
#![warn(clippy::manual_non_exhaustive)]
#![warn(clippy::manual_range_contains)]
#![warn(clippy::manual_saturating_arithmetic)]
#![warn(clippy::map_clone)]
#![warn(clippy::map_collect_result_unit)]
#![warn(clippy::match_like_matches_macro)]
#![warn(clippy::match_overlapping_arm)]
#![warn(clippy::match_ref_pats)]
#![warn(clippy::match_result_ok)]
#![warn(clippy::mem_replace_option_with_none)]
#![warn(clippy::mem_replace_with_default)]
#![warn(clippy::missing_safety_doc)]
#![warn(clippy::mixed_case_hex_literals)]
#![warn(clippy::module_inception)]
#![warn(clippy::must_use_unit)]
#![warn(clippy::mut_mutex_lock)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_doctest_main)]
#![warn(clippy::needless_late_init)]
#![warn(clippy::needless_parens_on_range_literals)]
#![warn(clippy::needless_range_loop)]
#![warn(clippy::needless_return)]
#![warn(clippy::neg_multiply)]
#![warn(clippy::new_ret_no_self)]
#![warn(clippy::new_without_default)]
#![warn(clippy::obfuscated_if_else)]
#![warn(clippy::ok_expect)]
#![warn(clippy::op_ref)]
#![warn(clippy::option_map_or_none)]
#![warn(clippy::partialeq_to_none)]
#![warn(clippy::print_literal)]
#![warn(clippy::print_with_newline)]
#![warn(clippy::println_empty_string)]
#![warn(clippy::ptr_arg)]
#![warn(clippy::ptr_eq)]
#![warn(clippy::question_mark)]
#![warn(clippy::redundant_closure)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::redundant_pattern)]
#![warn(clippy::redundant_pattern_matching)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::result_map_or_into_option)]
#![warn(clippy::result_unit_err)]
#![warn(clippy::same_item_push)]
#![warn(clippy::self_named_constructors)]
#![warn(clippy::should_implement_trait)]
#![warn(clippy::single_char_add_str)]
#![warn(clippy::single_component_path_imports)]
#![warn(clippy::single_match)]
#![warn(clippy::string_extend_chars)]
#![warn(clippy::tabs_in_doc_comments)]
#![warn(clippy::to_digit_is_some)]
#![warn(clippy::toplevel_ref_arg)]
#![warn(clippy::trim_split_whitespace)]
#![warn(clippy::uninlined_format_args)]
#![warn(clippy::unnecessary_fold)]
#![warn(clippy::unnecessary_lazy_evaluations)]
#![warn(clippy::unnecessary_mut_passed)]
#![warn(clippy::unnecessary_owned_empty_strings)]
#![warn(clippy::unsafe_removed_from_name)]
#![warn(clippy::unused_unit)]
#![warn(clippy::unusual_byte_groupings)]
#![warn(clippy::unwrap_or_else_default)]
#![warn(clippy::upper_case_acronyms)]
#![warn(clippy::while_let_on_iterator)]
#![warn(clippy::write_literal)]
#![warn(clippy::write_with_newline)]
#![warn(clippy::writeln_empty_string)]
#![warn(clippy::wrong_self_convention)]
#![warn(clippy::zero_ptr)]
//  suspicious
#![warn(clippy::almost_complete_letter_range)]
#![warn(clippy::await_holding_invalid_type)]
#![warn(clippy::await_holding_lock)]
#![warn(clippy::await_holding_refcell_ref)]
#![warn(clippy::blanket_clippy_restriction_lints)]
#![warn(clippy::cast_abs_to_unsigned)]
#![warn(clippy::cast_enum_constructor)]
#![warn(clippy::cast_enum_truncation)]
#![warn(clippy::cast_nan_to_int)]
#![warn(clippy::cast_slice_from_raw_parts)]
#![warn(clippy::crate_in_macro_def)]
#![warn(clippy::drop_non_drop)]
#![warn(clippy::duplicate_mod)]
#![warn(clippy::empty_loop)]
#![warn(clippy::float_equality_without_abs)]
#![warn(clippy::forget_non_drop)]
#![warn(clippy::from_raw_with_void_ptr)]
#![warn(clippy::let_underscore_future)]
#![warn(clippy::misnamed_getters)]
#![warn(clippy::misrefactored_assign_op)]
#![warn(clippy::multi_assignments)]
#![warn(clippy::mut_range_bound)]
#![warn(clippy::mutable_key_type)]
#![warn(clippy::no_effect_replace)]
#![warn(clippy::octal_escapes)]
#![warn(clippy::print_in_format_impl)]
#![warn(clippy::rc_clone_in_vec_init)]
#![warn(clippy::suspicious_arithmetic_impl)]
#![warn(clippy::suspicious_assignment_formatting)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_map)]
#![warn(clippy::suspicious_op_assign_impl)]
#![warn(clippy::suspicious_to_owned)]
#![warn(clippy::suspicious_unary_op_formatting)]
#![warn(clippy::swap_ptr_to_ref)]
#![warn(clippy::unchecked_duration_subtraction)]
// rustc
//  default=allow
#![warn(absolute_paths_not_starting_with_crate)]
#![allow(box_pointers)]
#![warn(rustc::elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
// #![warn(ffi_unwind_calls)]
#![warn(fuzzy_provenance_casts)]
#![warn(keyword_idents)]
#![warn(let_underscore_drop)]
#![warn(lossy_provenance_casts)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![allow(missing_docs)] // todo
#![warn(must_not_suspend)]
#![warn(rustc::non_ascii_idents)]
#![warn(non_exhaustive_omitted_patterns)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rustc::rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(unstable_features)]
#![warn(rustc::unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(unused_tuple_struct_fields)]
#![warn(variant_size_differences)]
//  default=warn
#![warn(anonymous_parameters)]
#![warn(array_into_iter)]
#![warn(asm_sub_register)]
#![warn(bad_asm_style)]
#![warn(bare_trait_objects)]
#![warn(bindings_with_variant_name)]
#![warn(break_with_label_and_loop)]
#![warn(clashing_extern_declarations)]
#![warn(coherence_leak_check)]
// #![warn(confusable_idents)]
#![warn(const_evaluatable_unchecked)]
#![warn(const_item_mutation)]
#![warn(dead_code)]
#![warn(deprecated)]
#![warn(deprecated_where_clause_location)]
#![warn(deref_into_dyn_supertrait)]
#![warn(deref_nullptr)]
#![warn(drop_bounds)]
#![warn(duplicate_macro_attributes)]
#![warn(dyn_drop)]
#![warn(ellipsis_inclusive_range_patterns)]
#![warn(exported_private_dependencies)]
#![warn(for_loops_over_fallibles)]
#![warn(forbidden_lint_groups)]
#![warn(function_item_references)]
#![warn(illegal_floating_point_literal_pattern)]
#![warn(implied_bounds_entailment)]
#![warn(improper_ctypes)]
#![warn(improper_ctypes_definitions)]
#![allow(incomplete_features)]
#![warn(indirect_structural_match)]
#![warn(inline_no_sanitize)]
#![warn(invalid_doc_attributes)]
#![warn(invalid_value)]
#![warn(irrefutable_let_patterns)]
#![warn(large_assignments)]
#![warn(late_bound_lifetime_arguments)]
#![warn(legacy_derive_helpers)]
// #![warn(mixed_script_confusables)]
#![warn(named_arguments_used_positionally)]
#![warn(no_mangle_generic_items)]
#![warn(non_camel_case_types)]
#![warn(non_fmt_panics)]
#![warn(non_shorthand_field_patterns)]
#![warn(non_snake_case)]
#![warn(non_upper_case_globals)]
#![warn(nontrivial_structural_match)]
#![warn(opaque_hidden_inferred_bound)]
#![warn(overlapping_range_endpoints)]
#![warn(path_statements)]
#![warn(private_in_public)]
#![warn(redundant_semicolons)]
#![warn(renamed_and_removed_lints)]
#![warn(repr_transparent_external_private_fields)]
#![warn(semicolon_in_expressions_from_macros)]
#![warn(special_module_name)]
#![warn(stable_features)]
#![warn(suspicious_auto_trait_impls)]
#![warn(temporary_cstring_as_ptr)]
#![warn(trivial_bounds)]
#![warn(type_alias_bounds)]
#![warn(tyvar_behind_raw_pointer)]
#![warn(rustc::uncommon_codepoints)]
#![warn(unconditional_recursion)]
//#![warn(undefined_naked_function_abi)]
#![warn(unexpected_cfgs)]
#![warn(unfulfilled_lint_expectations)]
#![warn(ungated_async_fn_track_caller)]
#![warn(uninhabited_static)]
#![warn(unknown_lints)]
#![warn(unnameable_test_items)]
#![warn(unreachable_code)]
#![warn(unreachable_patterns)]
#![warn(unstable_name_collisions)]
#![warn(unstable_syntax_pre_expansion)]
#![warn(unsupported_calling_conventions)]
#![warn(unused_allocation)]
#![warn(unused_assignments)]
#![warn(unused_attributes)]
#![warn(unused_braces)]
#![warn(unused_comparisons)]
#![warn(unused_doc_comments)]
#![warn(unused_features)]
#![warn(unused_imports)]
#![warn(unused_labels)]
#![warn(unused_macros)]
#![warn(unused_must_use)]
#![warn(unused_mut)]
#![warn(unused_parens)]
#![warn(unused_unsafe)]
#![warn(unused_variables)]
#![warn(warnings)]
#![warn(where_clauses_object_safety)]
#![warn(while_true)]
//  default=deny
#![deny(ambiguous_associated_items)]
#![deny(arithmetic_overflow)]
#![deny(cenum_impl_drop_cast)]
#![deny(conflicting_repr_hints)]
#![deny(deprecated_cfg_attr_crate_type_name)]
#![deny(enum_intrinsics_non_enums)]
#![deny(rustc::ill_formed_attribute_input)]
#![deny(incomplete_include)]
#![deny(ineffective_unstable_trait_impl)]
#![deny(invalid_atomic_ordering)]
#![deny(invalid_type_param_default)]
#![deny(let_underscore_lock)]
#![deny(rustc::macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![deny(missing_fragment_specifier)]
#![deny(mutable_transmutes)]
#![deny(named_asm_labels)]
#![deny(no_mangle_const_items)]
#![deny(order_dependent_trait_objects)]
#![deny(overflowing_literals)]
#![deny(patterns_in_fns_without_body)]
#![deny(proc_macro_back_compat)]
#![deny(proc_macro_derive_resolution_fallback)]
#![deny(pub_use_of_private_extern_crate)]
#![deny(soft_unstable)]
//#![deny(test_unstable_lint)]
#![deny(text_direction_codepoint_in_comment)]
#![deny(text_direction_codepoint_in_literal)]
#![deny(unaligned_references)]
#![deny(unconditional_panic)]
#![deny(rustc::unknown_crate_types)]
#![deny(useless_deprecated)]
// rustdoc
#![warn(broken_intra_doc_links)]
#![warn(private_intra_doc_links)]
//#![warn(missing_docs)]
#![warn(missing_crate_level_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]
#![warn(invalid_codeblock_attributes)]
#![warn(invalid_html_tags)]
#![warn(rustdoc::invalid_rust_codeblocks)]
#![warn(rustdoc::bare_urls)]
#![feature(try_trait_v2)]
#![feature(let_chains)]
#![feature(adt_const_params)]
#![feature(strict_provenance)]
#![feature(must_not_suspend)]
#![feature(round_char_boundary)]
#![feature(non_exhaustive_omitted_patterns_lint)]
#![feature(custom_test_frameworks)]
#![feature(default_free_fn)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(unboxed_closures)]
#![feature(lint_reasons)]
//! A good client for browsing reddit without the cancer

use post::get_post_comments;

/// Handles browse subcommand
pub mod browse;
/// Handles CLI commands
pub mod command;
/// Handles comments
pub mod comment;
/// Handles posts
pub mod post;
/// Handles settings
pub mod settings;
/// Handles subreddits
pub mod subreddit;
#[cfg(test)]
mod tests;
/// Handles users
pub mod user;

use {
	crate::{
		browse::*,
		command::*,
		settings::*,
	},
	clap::Parser,
	enum_map::enum_map,
	regex::Regex,
	roux::Subreddit,
	screwit::{
		self,
		FilterAction,
		RegexPolicy,
		Severity,
		SeverityPolicy,
		SubredditPolicies,
	},
	tokio,
};

#[tokio::main]
async fn main()
{
	let args = command::Cli::parse();
	match args.command
	{
		Command::Browse { object } => browse(object),
		Command::Setting { object } => setting(object),
		Command::Test { value } => println!("{:?}", value),
		Command::Example {} => example_filter().await,
	}
}

async fn example_filter()
{
	let policy: SeverityPolicy = enum_map! {
		Severity::Ok => FilterAction::Ignore,
		Severity::Questionable => FilterAction::Tag,
		Severity::Awful => FilterAction::Filter,
		Severity::Scourge => FilterAction::Report,

	};

	#[rustfmt::skip]
	let subreddit_filter: SubredditPolicies = vec!
		[
			RegexPolicy { regex: Regex::new(          "conservative").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(            "conspiracy").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(                  "sino").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(     "ConservativeMemes").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(         "conservatives").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(            "republican").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "walkaway").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(     "louderwithcrowder").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(      "shitpoliticssays").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(                "progun").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(               "prolife").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(           "gunpolitics").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "asktrumpsupporters").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new( "socialjusticeinaction").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(          "goldandblack").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "shitstatistssay").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(           "libertarian").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "libertarianmeme").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new( "coronaviruscirclejerk").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "anarcho_capitalism").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "firearms").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(           "nonewnormal").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "theleftcantmeme").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(        "jordanpeterson").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "lockdownskepticism").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "centrist").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new( "actualpublicfreakouts").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "protectandserve").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(        "kotakuinaction").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(      "moderatepolitics").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "conspiracy_commons").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(  "trueunpopularopinion").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "joerogan").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(            "mensrights").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(                 "memes").expect(""), severity: Severity::Ok },
		];
	let subreddit = Subreddit::new("gaming");
	let new = subreddit.latest(1, None).await;
	let article_id = &new.unwrap().data.children.first().unwrap().data.id.clone();
	if let Ok(comments) = get_post_comments(&subreddit, article_id, &subreddit_filter, &policy).await
	{
		println!("{}", comments);
	}
}
