use super::*;

fn node_count(a: &NodeRef) -> u32 {
    let a = a.data();
    match a.value() {
        &Value::Array(ref a) => {
            a.iter().fold(1, |c, n| c + node_count(n))
        }
        &Value::Object(ref a) => {
            a.values().fold(1, |c, n| c + node_count(n))
        }
        _ => 1
    }
}

fn node_distance(a: &NodeRef, b: &NodeRef) -> (u32, u32) {
    let mut dist: u32 = 0;
    let mut count: u32 = 1;

    let a = a.data();
    let b = b.data();

    match (a.value(), b.value()) {
        (&Value::Null, &Value::Null) => {}
        (&Value::Boolean(a), &Value::Boolean(b)) => {
            if a != b {
                dist = 1;
            }
        }
        (&Value::Integer(a), &Value::Integer(b)) => {
            if a != b {
                dist = 1;
            }
        }
        (&Value::Float(a), &Value::Float(b)) => {
            if a.to_bits() != b.to_bits() {
                dist = 1;
            }
        }
        (&Value::Binary(ref a), &Value::Binary(ref b)) => {
            if a != b {
                dist = 1;
            }
        }
        (&Value::String(ref a), &Value::String(ref b)) => {
            if a != b {
                dist = 1;
            }
        }
        (&Value::Array(ref a), &Value::Array(ref b)) => {
            for (a, b) in a.iter().zip(b.iter()) {
                let (d, c) = node_distance(&a, &b);
                dist += d;
                count += c;
            }
            if a.len() > b.len() {
                for a in a.iter().skip(b.len() - 1) {
                    let c = node_count(a);
                    dist += c;
                    count += c;
                }
            } else if a.len() < b.len() {
                for b in b.iter().skip(a.len() - 1) {
                    let c = node_count(b);
                    dist += c;
                    count += c;
                }
            }
        }
        (&Value::Object(ref a), &Value::Object(ref b)) => {
            use std::collections::HashSet;

            let mut keys: HashSet<&Symbol> = HashSet::with_capacity(a.len() + b.len());
            for k in a.keys().chain(b.keys()) {
                keys.insert(k);
            }
            for &k in keys.iter() {
                match (a.get(k), b.get(k)) {
                    (Some(a), Some(b)) => {
                        let (d, c) = node_distance(a, b);
                        dist += d;
                        count += c;
                    }
                    (Some(a), None) | (None, Some(a)) => {
                        let c = node_count(a);
                        dist += c;
                        count += c;
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        (&Value::Object(ref a), &Value::Array(ref b)) | (&Value::Array(ref b), &Value::Object(ref a)) => {
            dist = 1;
            for a in a.values() {
                let c = node_count(a);
                dist += c;
                count += c;
            }
            for b in b.iter() {
                let c = node_count(b);
                dist += c;
                count += c;
            }
        }
        (&Value::Object(ref a), _) | (_, &Value::Object(ref a)) => {
            dist = 1;
            for a in a.values() {
                let c = node_count(a);
                dist += c;
                count += c;
            }
        }
        (&Value::Array(ref a), _) | (_, &Value::Array(ref a)) => {
            dist = 1;
            for a in a.iter() {
                let c = node_count(a);
                dist += c;
                count += c;
            }
        }
        (_, _) => {
            dist = 1;
        }
    }

    (dist, count)
}

pub fn distance(a: &NodeRef, b: &NodeRef, min_count: Option<u32>) -> f64 {
    if a.is_ref_eq(b) {
        0.0
    } else {
        let (d, c) = node_distance(a, b);
        if let Some(min) = min_count {
            if c <= min {
                return 1.0;
            }
        }
        d as f64 / c as f64
    }
}