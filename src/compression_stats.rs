use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// 压缩统计配置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionStatsConfig {
    /// 是否启用压缩统计
    pub enabled: bool,
    /// 是否打印详细日志
    pub verbose_logging: bool,
    /// 统计数据保存时长（秒）
    pub retention_seconds: u64,
}

impl Default for CompressionStatsConfig {
    fn default() -> Self {
        Self {
            enabled: false, // 默认关闭以提高性能
            verbose_logging: false,
            retention_seconds: 3600, // 1小时
        }
    }
}

/// 单个事件的压缩统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCompressionStats {
    pub event_type: String,
    pub struct_size: usize,
    pub bincode_size: usize,
    pub json_size: usize,
    pub serialized_size: usize,
    pub compression_results: Vec<(String, usize, f64)>, // (压缩方法, 压缩后大小, 节省比例)
    pub final_compressed_size: usize,
    pub used_compression: String,
    pub total_saving: i32,
    pub total_ratio: f64,
    pub timestamp: u64,
}

/// 聚合统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedStats {
    pub event_type: String,
    pub total_events: u64,
    pub avg_struct_size: f64,
    pub avg_compression_ratio: f64,
    pub best_compression_method: String,
    pub total_bytes_saved: u64,
    pub last_updated: u64,
}

/// 压缩统计工具
pub struct CompressionStatsCollector {
    config: CompressionStatsConfig,
    event_stats: Arc<Mutex<Vec<EventCompressionStats>>>,
    aggregated_stats: Arc<Mutex<HashMap<String, AggregatedStats>>>,
}

impl CompressionStatsCollector {
    /// 创建新的压缩统计收集器
    pub fn new(config: CompressionStatsConfig) -> Self {
        Self {
            config,
            event_stats: Arc::new(Mutex::new(Vec::new())),
            aggregated_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 是否启用统计
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// 记录压缩统计数据
    pub fn record_compression_stats(
        &self,
        event_type: String,
        struct_size: usize,
        bincode_size: usize,
        json_size: usize,
        serialized_size: usize,
        compression_results: Vec<(String, usize, f64)>,
        final_compressed_size: usize,
        used_compression: String,
    ) {
        if !self.config.enabled {
            return;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let total_saving = struct_size as i32 - final_compressed_size as i32;
        let total_ratio = if struct_size > 0 {
            (total_saving as f64 / struct_size as f64) * 100.0
        } else {
            0.0
        };

        let stats = EventCompressionStats {
            event_type: event_type.clone(),
            struct_size,
            bincode_size,
            json_size,
            serialized_size,
            compression_results: compression_results.clone(),
            final_compressed_size,
            used_compression: used_compression.clone(),
            total_saving,
            total_ratio,
            timestamp,
        };

        // 记录详细统计
        if let Ok(mut event_stats) = self.event_stats.lock() {
            event_stats.push(stats.clone());
            
            // 清理过期数据
            let cutoff_time = timestamp - self.config.retention_seconds;
            event_stats.retain(|s| s.timestamp >= cutoff_time);
        }

        // 更新聚合统计
        if let Ok(mut aggregated) = self.aggregated_stats.lock() {
            let entry = aggregated.entry(event_type.clone()).or_insert_with(|| {
                AggregatedStats {
                    event_type: event_type.clone(),
                    total_events: 0,
                    avg_struct_size: 0.0,
                    avg_compression_ratio: 0.0,
                    best_compression_method: used_compression.clone(),
                    total_bytes_saved: 0,
                    last_updated: timestamp,
                }
            });

            entry.total_events += 1;
            entry.avg_struct_size = (entry.avg_struct_size * (entry.total_events - 1) as f64 + struct_size as f64) / entry.total_events as f64;
            entry.avg_compression_ratio = (entry.avg_compression_ratio * (entry.total_events - 1) as f64 + total_ratio) / entry.total_events as f64;
            
            if total_saving > 0 {
                entry.total_bytes_saved += total_saving as u64;
            }
            
            entry.last_updated = timestamp;

            // 找出最佳压缩方法
            if let Some((best_method, _, _)) = compression_results.iter().min_by_key(|(_, size, _)| *size) {
                entry.best_compression_method = best_method.clone();
            }
        }

        // 如果启用了详细日志，打印统计信息
        if self.config.verbose_logging {
            self.print_detailed_stats(&stats);
        }
    }

    /// 打印详细统计信息
    fn print_detailed_stats(&self, stats: &EventCompressionStats) {
        println!("📊 {} 数据处理统计:", stats.event_type);
        println!("  结构体大小: {} bytes (内存中)", stats.struct_size);
        
        println!("  序列化格式比较:");
        println!("    Bincode: {} bytes", stats.bincode_size);
        println!("    JSON: {} bytes", stats.json_size);
        
        let best_format = if stats.bincode_size <= stats.json_size { "Bincode" } else { "JSON" };
        println!("    最优选择: {} ({} bytes)", best_format, stats.serialized_size);

        // 序列化效果分析
        if stats.serialized_size != stats.struct_size {
            let serialization_change = if stats.serialized_size > stats.struct_size {
                let increase = stats.serialized_size - stats.struct_size;
                format!("+{} bytes (+{:.1}%)", increase, (increase as f64 / stats.struct_size as f64) * 100.0)
            } else {
                let decrease = stats.struct_size - stats.serialized_size;
                format!("-{} bytes (-{:.1}%)", decrease, (decrease as f64 / stats.struct_size as f64) * 100.0)
            };
            println!("  序列化效果: {} bytes ({})", stats.serialized_size, serialization_change);
        } else {
            println!("  序列化效果: {} bytes (无变化)", stats.serialized_size);
        }

        println!("  压缩方法比较 (原始序列化数据: {} bytes):", stats.serialized_size);
        for (compression_name, compressed_size, savings) in &stats.compression_results {
            if *compressed_size == stats.serialized_size {
                println!("    {}: {} bytes (无压缩)", compression_name, compressed_size);
            } else {
                let saved_bytes = stats.serialized_size - compressed_size;
                println!("    {}: {} bytes (节约 {} bytes, {:.1}%)", 
                         compression_name, compressed_size, saved_bytes, savings);
            }
        }

        println!("  实际使用压缩: {} ({} bytes)", stats.used_compression, stats.final_compressed_size);

        if stats.total_saving > 0 {
            println!("  总体节约: {} bytes ({:.1}%)", stats.total_saving, stats.total_ratio);
        } else {
            println!("  总体增加: {} bytes ({:.1}%)", -stats.total_saving, -stats.total_ratio);
        }

        // 性能提示
        if let Some((best_method, best_size, _)) = stats.compression_results.iter().min_by_key(|(_, size, _)| *size) {
            if best_method != &stats.used_compression {
                let potential_savings = stats.final_compressed_size as i32 - *best_size as i32;
                if potential_savings > 0 {
                    println!("  💡 理论最优: {} 可额外节约 {} bytes，但 {} 提供最佳速度/压缩平衡", 
                             best_method, potential_savings, stats.used_compression);
                }
            } else {
                println!("  ✅ {} 已是此事件的最优压缩选择", stats.used_compression);
            }
        }

        println!("  ========================");
    }

    /// 获取聚合统计数据
    pub fn get_aggregated_stats(&self) -> HashMap<String, AggregatedStats> {
        if !self.config.enabled {
            return HashMap::new();
        }

        self.aggregated_stats.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// 获取指定事件类型的最近统计数据
    pub fn get_recent_stats(&self, event_type: &str, limit: usize) -> Vec<EventCompressionStats> {
        if !self.config.enabled {
            return Vec::new();
        }

        let event_stats = self.event_stats.lock().unwrap_or_else(|e| e.into_inner());
        event_stats
            .iter()
            .filter(|s| s.event_type == event_type)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// 清理过期统计数据
    pub fn cleanup_expired_stats(&self) {
        if !self.config.enabled {
            return;
        }

        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() - self.config.retention_seconds;

        if let Ok(mut event_stats) = self.event_stats.lock() {
            event_stats.retain(|s| s.timestamp >= cutoff_time);
        }
    }

    /// 打印总体统计报告
    pub fn print_summary_report(&self) {
        if !self.config.enabled {
            return;
        }

        let aggregated = self.get_aggregated_stats();
        if aggregated.is_empty() {
            println!("📊 压缩统计报告: 暂无数据");
            return;
        }

        println!("📊 压缩统计总结报告");
        println!("==========================================");
        
        for (event_type, stats) in aggregated.iter() {
            println!("🎯 事件类型: {}", event_type);
            println!("  总处理量: {} 个事件", stats.total_events);
            println!("  平均大小: {:.1} bytes", stats.avg_struct_size);
            println!("  平均压缩率: {:.1}%", stats.avg_compression_ratio);
            println!("  最佳压缩方法: {}", stats.best_compression_method);
            println!("  累计节省: {} bytes", stats.total_bytes_saved);
            println!("  ------------------------------------------");
        }
    }

    /// 更新配置
    pub fn update_config(&mut self, config: CompressionStatsConfig) {
        self.config = config;
    }
}