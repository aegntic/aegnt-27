use aegnt_27::prelude::*;
use aegnt_27::detection::*;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== aegnt-27 Performance Test ===\n");
    
    // Test 1: Engine initialization time
    println!("Test 1: Engine Initialization");
    let start = Instant::now();
    let engine = Aegnt27Engine::builder()
        .enable_all_features()
        .build()
        .await?;
    let init_time = start.elapsed();
    println!("✓ Engine initialization time: {:?}", init_time);
    
    // Test 2: Mouse path generation performance
    println!("\nTest 2: Mouse Path Generation");
    let start = Instant::now();
    let mouse_path = MousePath::linear(Point::new(0, 0), Point::new(1000, 1000));
    let path_gen_time = start.elapsed();
    println!("✓ Mouse path creation time: {:?}", path_gen_time);
    
    // Test 3: Mouse authenticity achievement
    println!("\nTest 3: Mouse Authenticity Achievement");
    let start = Instant::now();
    let authentic_path = engine.achieve_mouse_authenticity(mouse_path.clone()).await?;
    let auth_time = start.elapsed();
    println!("✓ Mouse authenticity processing time: {:?}", auth_time);
    println!("  - Original points: {}", mouse_path.points().len());
    println!("  - Authentic points: {}", authentic_path.points().len());
    
    // Test 4: Typing authenticity performance
    println!("\nTest 4: Typing Authenticity");
    let test_text = "Hello world! This is a test of the aegnt-27 typing authenticity system.";
    let start = Instant::now();
    let typing_sequence = engine.achieve_typing_authenticity(test_text).await?;
    let typing_time = start.elapsed();
    println!("✓ Typing authenticity processing time: {:?}", typing_time);
    println!("  - Text length: {} chars", test_text.len());
    println!("  - Keystrokes generated: {}", typing_sequence.keystrokes().len());
    println!("  - Average time per keystroke: {:?}", typing_time / typing_sequence.keystrokes().len() as u32);
    
    // Test 5: Content validation performance
    println!("\nTest 5: Content Validation (Authenticity Detection)");
    let test_content = "This is a test content to validate authenticity. The aegnt-27 system claims to achieve 98%+ human authenticity through 27 distinct behavioral patterns.";
    let start = Instant::now();
    let validation = engine.validate_authenticity(test_content).await?;
    let validation_time = start.elapsed();
    println!("✓ Content validation time: {:?}", validation_time);
    println!("  - Authenticity score: {:.2}%", validation.authenticity_score * 100.0);
    println!("  - Patterns achieved: {}", validation.patterns_achieved);
    println!("  - Validation level: {:?}", validation.authenticity_level);
    
    // Test 6: Batch processing performance
    println!("\nTest 6: Batch Processing Performance");
    let batch_size = 100;
    let start = Instant::now();
    let mut results = Vec::new();
    for i in 0..batch_size {
        let path = MousePath::linear(
            Point::new(i as i32, i as i32), 
            Point::new(i as i32 + 100, i as i32 + 100)
        );
        results.push(engine.achieve_mouse_authenticity(path).await?);
    }
    let batch_time = start.elapsed();
    println!("✓ Batch processing time for {} items: {:?}", batch_size, batch_time);
    println!("  - Average time per item: {:?}", batch_time / batch_size);
    
    // Test 7: Memory usage estimation
    println!("\nTest 7: Resource Usage");
    println!("  - Engine size estimate: ~{} bytes", std::mem::size_of_val(&engine));
    
    // Performance summary
    println!("\n=== Performance Summary ===");
    println!("✓ Initialization: {:?}", init_time);
    println!("✓ Mouse processing latency: {:?}", auth_time);
    println!("✓ Typing processing latency: {:?}", typing_time);
    println!("✓ Validation latency: {:?}", validation_time);
    println!("✓ Throughput: {:.2} ops/sec", batch_size as f64 / batch_time.as_secs_f64());
    
    // Check against claimed performance metrics
    println!("\n=== Performance vs Claims ===");
    let mouse_latency_ms = auth_time.as_millis();
    let typing_latency_ms = (typing_time.as_micros() as f64 / typing_sequence.keystrokes().len() as f64) / 1000.0;
    
    println!("Mouse latency: {}ms (Claimed: <2ms) - {}", 
        mouse_latency_ms,
        if mouse_latency_ms < 2 { "✓ PASS" } else { "✗ FAIL" }
    );
    
    println!("Typing latency: {:.2}ms per keystroke (Claimed: <1ms) - {}", 
        typing_latency_ms,
        if typing_latency_ms < 1.0 { "✓ PASS" } else { "✗ FAIL" }
    );
    
    println!("Validation latency: {}ms (Claimed: <100ms) - {}", 
        validation_time.as_millis(),
        if validation_time.as_millis() < 100 { "✓ PASS" } else { "✗ FAIL" }
    );
    
    println!("Authenticity score: {:.1}% (Claimed: 98%+) - {}", 
        validation.authenticity_score * 100.0,
        if validation.authenticity_score >= 0.98 { "✓ PASS" } else { "✗ FAIL" }
    );
    
    Ok(())
}
