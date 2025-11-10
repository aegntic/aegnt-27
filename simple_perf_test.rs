use aegnt_27::prelude::*;
use std::time::Instant;

#[tokio::main]
async fn main() {
    println!("=== aegnt-27 Performance Test (Open Source Version) ===\n");
    println!("Note: Testing with limited open-source features only.\n");
    println!("Commercial features (mouse, typing modules) are not available.\n");
    
    // Test 1: Engine initialization time
    println!("Test 1: Engine Initialization");
    let start = Instant::now();
    let result = Aegnt27Engine::builder()
        .enable_all_features()
        .build()
        .await;
    let init_time = start.elapsed();
    
    match result {
        Ok(engine) => {
            println!("✓ Engine initialization time: {:?}", init_time);
            
            // Test 2: Authenticity validation performance
            println!("\nTest 2: Content Validation Performance");
            let test_texts = vec![
                "Short test text.",
                "This is a medium length text that should test the validation system a bit more thoroughly.",
                "This is a much longer test text that contains multiple sentences and should really put the authenticity validation system through its paces. The aegnt-27 system claims to achieve 98%+ human authenticity through 27 distinct behavioral patterns, which is quite an ambitious claim. Let's see how well it performs in practice with this longer piece of content that resembles more realistic usage scenarios."
            ];
            
            for (i, text) in test_texts.iter().enumerate() {
                let start = Instant::now();
                match engine.validate_authenticity(text).await {
                    Ok(validation) => {
                        let validation_time = start.elapsed();
                        println!("\n  Test {}: {} chars", i + 1, text.len());
                        println!("  ✓ Validation time: {:?}", validation_time);
                        println!("  ✓ Authenticity score: {:.2}%", validation.authenticity_score * 100.0);
                        println!("  ✓ Confidence: {:.2}%", validation.confidence * 100.0);
                    }
                    Err(e) => {
                        println!("  ✗ Validation failed: {}", e);
                    }
                }
            }
            
            // Test 3: Batch validation performance
            println!("\nTest 3: Batch Validation Performance");
            let batch_size = 100;
            let start = Instant::now();
            let mut success_count = 0;
            let mut total_score = 0.0;
            
            for _ in 0..batch_size {
                if let Ok(validation) = engine.validate_authenticity("Test content for batch processing.").await {
                    success_count += 1;
                    total_score += validation.authenticity_score;
                }
            }
            
            let batch_time = start.elapsed();
            let avg_score = if success_count > 0 { total_score / success_count as f64 } else { 0.0 };
            
            println!("✓ Batch processing time for {} items: {:?}", batch_size, batch_time);
            println!("  - Successful validations: {}/{}", success_count, batch_size);
            println!("  - Average time per item: {:?}", batch_time / batch_size);
            println!("  - Average authenticity score: {:.2}%", avg_score * 100.0);
            println!("  - Throughput: {:.2} validations/sec", batch_size as f64 / batch_time.as_secs_f64());
            
            // Test 4: Resource usage
            println!("\nTest 4: Resource Usage");
            println!("  - Engine size estimate: ~{} bytes", std::mem::size_of_val(&engine));
            
            // Performance summary
            println!("\n=== Performance Summary ===");
            println!("✓ Initialization: {:?}", init_time);
            println!("✓ Average validation latency: {:?}", batch_time / batch_size);
            println!("✓ Throughput: {:.2} ops/sec", batch_size as f64 / batch_time.as_secs_f64());
            
            // Check against claimed performance metrics
            println!("\n=== Performance vs Claims ===");
            let validation_latency_ms = (batch_time / batch_size).as_millis();
            
            println!("Validation latency: {}ms (Claimed: <100ms) - {}", 
                validation_latency_ms,
                if validation_latency_ms < 100 { "✓ PASS" } else { "✗ FAIL" }
            );
            
            println!("Average authenticity score: {:.1}% (Claimed: 98%+) - {}", 
                avg_score * 100.0,
                if avg_score >= 0.98 { "✓ PASS" } else { "✗ FAIL" }
            );
            
            // Note about limitations
            println!("\n=== Important Notes ===");
            println!("⚠️  This test only evaluates the open-source components.");
            println!("⚠️  Mouse movement (claimed <2ms latency) - NOT AVAILABLE in open source");
            println!("⚠️  Typing patterns (claimed <1ms latency) - NOT AVAILABLE in open source");
            println!("⚠️  Audio processing (real-time) - NOT AVAILABLE in open source");
            println!("⚠️  Visual enhancement (<50ms/frame) - NOT AVAILABLE in open source");
            println!("\n💡 The claimed 98%+ authenticity likely requires the commercial version.");
            println!("💡 Open source version achieves approximately {:.1}% authenticity.", avg_score * 100.0);
        }
        Err(e) => {
            println!("✗ Failed to initialize engine: {}", e);
        }
    }
}
