// Copyright © Aptos Foundation

use aptos_metrics_core::{register_int_counter, IntCounter};
use once_cell::sync::Lazy;

// OVERALL METRICS

/// Number of times a given processor has been invoked
pub static PARSER_INVOCATIONS_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_processor_invocation_count",
        "Number of times the parser has been invoked",
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has completed successfully
pub static PARSER_SUCCESSES_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_processor_success_count",
        "Number of times the parser has completed successfully",
    )
    .unwrap()
});

/// Number of times the PubSub subscription stream has been reset
pub static PUBSUB_STREAM_RESET_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parse_pubsub_stream_reset_count",
        "Number of times the PubSub subscription stream has been reset",
    )
    .unwrap()
});

/// Number of times a PubSub message has successfully been ACK'd
pub static PUBSUB_ACK_SUCCESS_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_pubsub_ack_success_count",
        "Number of times a PubSub message has successfully been ACK'd",
    )
    .unwrap()
});

/// Number of URIs skipped because of matches on the URI skip list
pub static SKIP_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_skip_uri_count",
        "Number of URIs skipped because of matches on the URI skip list",
    )
    .unwrap()
});

// DEDUPLICATION METRICS

/// Number of times the NFT Metadata Crawler Parser has found a duplicate token URI
pub static DUPLICATE_TOKEN_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_duplicate_token_uri_count",
        "Number of times the NFT Metadata Crawler Parser has found a duplicate token URI"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has found a duplicate raw image URI
pub static DUPLICATE_RAW_IMAGE_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_duplicate_raw_image_uri_count",
        "Number of times the NFT Metadata Crawler Parser has found a duplicate raw image URI"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has found a duplicate raw animation URI
pub static DUPLICATE_RAW_ANIMATION_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_duplicate_raw_animation_uri_count",
        "Number of times the NFT Metadata Crawler Parser has found a duplicate raw animation URI"
    )
    .unwrap()
});

// URI PARSER METRICS

/// Number of times the NFT Metadata Crawler Parser has invocated the URI Parser
pub static PARSE_URI_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_uri_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the URI Parser"
    )
    .unwrap()
});

/// Number of times the URI Parser has found an arweave URI
pub static PARSE_URI_FOUND_ARWEAVE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_uri_found_arweave_count",
        "Number of times the NFT Metadata Crawler Parser has found an Arweave URI"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been able to parse a URI to use the dedicated IPFS gateway
pub static SUCCESSFULLY_PARSED_IPFS_URI_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_ipfs_uri_count",
        "Number of times the NFT Metadata Crawler Parser has been able to parse a URI to use the dedicated IPFS gateway"
    )
    .unwrap()
});

// JSON PARSER METRICS

/// Number of times the NFT Metadata Crawler has invocated the JSON Parser
pub static PARSE_JSON_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the JSON Parser"
    )
    .unwrap()
});

/// Number of times the JSON Parser has been unable to parse a JSON because it was too large
pub static PARSE_JSON_FILE_TOO_LARGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_file_too_large_count",
        "Number of times the JSON Parser has been unable to parse a JSON because it was too large"
    )
    .unwrap()
});

/// Number of times the JSON Parser has been unable to parse a JSON because an image was found instead
pub static PARSE_JSON_FILE_FOUND_IMAGE_INSTEAD_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_parse_json_file_found_image_instead",
        "Number of times the JSON Parser has been unable to parse a JSON because it found an image instead"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has been able to parse a JSON
pub static SUCCESSFULLY_PARSED_JSON_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_successfully_parsed_json_count",
        "Number of times the NFT Metadata Crawler Parser has been able to parse a JSON"
    )
    .unwrap()
});

// IMAGE OPTIMIZER METRICS

/// Number of times the NFT Metadata Crawler Parser has invocated the Image Optimizer for an image
pub static OPTIMIZE_IMAGE_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_image_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the Image Optimizer for an image"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been unable to optimize an image because it was too large
pub static OPTIMIZE_IMAGE_FILE_TOO_LARGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_image_file_too_large_count",
        "Number of times the Image Optimizer has been unable to optimize an image because it was too large"
    )
    .unwrap()
});

/// Number of times the NFT Metadata Crawler Parser has invocated the Image Optimizer for an animation
pub static OPTIMIZE_ANIMATION_INVOCATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_optimize_animation_invocation_count",
        "Number of times the NFT Metadata Crawler Parser has invocated the Image Optimizer for an animation"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been able to optimize an image
pub static SUCCESSFULLY_OPTIMIZED_IMAGE_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_successfully_optimized_image_count",
        "Number of times the Image Optimizer has been able to optimize an image"
    )
    .unwrap()
});

/// Number of times the Image Optimizer has been able to optimize an animation
pub static SUCCESSFULLY_OPTIMIZED_ANIMATION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "nft_metadata_crawler_parser_successfully_optimized_animation_count",
        "Number of times the Image Optimizer has been able to optimize an animation"
    )
    .unwrap()
});
