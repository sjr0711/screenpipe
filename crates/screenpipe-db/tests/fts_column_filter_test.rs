use screenpipe_db::{ContentType, DatabaseManager, OcrEngine};
use std::sync::Arc;

#[tokio::test]
async fn test_fts_column_filter_with_dots_and_quotes() {
    // Create in-memory DB
    let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
    
    // insert video chunk first
    let _ = db.insert_video_chunk("test.mp4", "test_device").await.unwrap();
    
    // insert frame with app_name containing dot
    let frame_id = db.insert_frame(
        "test_device",
        None,
        None,
        Some("zoom.us"), 
        Some("Zoom Window"),
        true,
        None
    ).await.unwrap();
    
    // insert ocr to populate FTS
    let _ = db.insert_ocr_text(frame_id, "some text", "{}", Arc::new(OcrEngine::Tesseract)).await.unwrap();

    // insert frame with app_name containing quote
    let frame_id2 = db.insert_frame(
        "test_device",
        None,
        None,
        Some("My \"App\""), 
        Some("Window"),
        true,
        None
    ).await.unwrap();
    let _ = db.insert_ocr_text(frame_id2, "some text 2", "{}", Arc::new(OcrEngine::Tesseract)).await.unwrap();

    // 1. Search for "zoom.us" (dot handling)
    let result = db.count_search_results(
        "", 
        ContentType::OCR,
        None,
        None,
        Some("zoom.us"), 
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ).await;
    
    assert!(result.is_ok(), "Search for 'zoom.us' failed: {:?}", result.err());
    assert_eq!(result.unwrap(), 1, "Should find 1 result for 'zoom.us'");

    // 2. Search for 'My "App"' (quote handling)
    let result = db.count_search_results(
        "", 
        ContentType::OCR,
        None,
        None,
        Some("My \"App\""), 
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ).await;
    
    assert!(result.is_ok(), "Search for 'My \"App\"' failed: {:?}", result.err());
    assert_eq!(result.unwrap(), 1, "Should find 1 result for 'My \"App\"'");
}
