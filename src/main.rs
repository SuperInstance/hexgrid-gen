use std::collections::{HashMap, HashSet, BTreeMap};
use std::env;
use std::fmt::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("hexgrid-gen — Eisenstein hex grid code generator");
        eprintln!("Commands: neighbors, disk, table, distances, verify, stats");
        eprintln!("Options: --radius N (default 50), --lang rust|c|python|js|json");
        std::process::exit(1);
    }
    let cmd = &args[1];
    let radius: i64 = args.iter().position(|a|a=="--radius").and_then(|i|args.get(i+1).and_then(|v|v.parse().ok())).unwrap_or(50);
    let lang: &str = args.iter().position(|a|a=="--lang").and_then(|i|args.get(i+1).map(|s|s.as_str())).unwrap_or("rust");

    match cmd.as_str() {
        "neighbors" => gen_neighbors(radius, lang),
        "disk" => gen_disk(radius, lang),
        "table" => gen_table(radius, lang),
        "distances" => gen_distances(radius, lang),
        "verify" => verify(radius),
        "stats" => stats(radius),
        _ => eprintln!("Unknown: {}", cmd),
    }
}

fn norm(a: i64, b: i64) -> i64 { a*a - a*b + b*b }

const D6: [(i64, i64); 6] = [(1,0),(-1,0),(0,1),(0,-1),(1,-1),(-1,1)];

fn disk(radius: i64) -> Vec<(i64, i64)> {
    let r2 = radius * radius;
    let mut pts = Vec::new();
    for a in -radius..=radius {
        for b in -radius..=radius {
            if norm(a, b) <= r2 { pts.push((a, b)); }
        }
    }
    pts
}

fn gen_neighbors(_radius: i64, lang: &str) {
    let mut o = String::new();
    match lang {
        "rust" => {
            o.push_str("/// D6 neighbor offsets for Eisenstein E12 hex grid\npub const D6_OFFSETS: [(i32, i32); 6] = [\n");
            for (da, db) in D6 { let _ = writeln!(o, "    ({}, {}),", da, db); }
            o.push_str("];\n\n/// Norm: a² - ab + b² (always non-negative integer)\n#[inline]\npub fn e12_norm(a: i32, b: i32) -> i64 {\n    let (aa, bb) = (a as i64, b as i64);\n    aa*aa - aa*bb + bb*bb\n}\n");
        }
        "c" => {
            o.push_str("/* D6 neighbor offsets — Eisenstein E12 */\n#ifndef HEXGRID_H\n#define HEXGRID_H\n#include <stdint.h>\n\n");
            o.push_str("static const int32_t d6_offsets[6][2] = {\n");
            for (da, db) in D6 { let _ = writeln!(o, "    {{{}, {}}},", da, db); }
            o.push_str("};\n\nstatic inline int64_t e12_norm(int32_t a, int32_t b) {\n    int64_t aa = a, bb = b;\n    return aa*aa - aa*bb + bb*bb;\n}\n\n#endif\n");
        }
        "python" => {
            o.push_str("# D6 neighbor offsets — Eisenstein E12\nD6_OFFSETS = [\n");
            for (da, db) in D6 { let _ = writeln!(o, "    ({}, {}),", da, db); }
            o.push_str("]\n\ndef e12_norm(a: int, b: int) -> int:\n    return a*a - a*b + b*b\n");
        }
        "json" => {
            o.push_str("{\"d6_offsets\": [\n");
            for (i, (da, db)) in D6.iter().enumerate() { let _ = writeln!(o, "  [{}, {}]{}", da, db, if i<5{","}else{""}); }
            o.push_str("]}\n");
        }
        _ => o.push_str("// Unsupported lang\n"),
    }
    print!("{}", o);
}

fn gen_disk(radius: i64, lang: &str) {
    let pts = disk(radius);
    let mut o = String::new();
    match lang {
        "json" => {
            let _ = writeln!(o, "{{\"radius\": {}, \"count\": {}, \"points\": [", radius, pts.len());
            for (i, (a, b)) in pts.iter().enumerate() {
                let _ = writeln!(o, "  {{\"a\": {}, \"b\": {}, \"norm\": {}}}{}", a, b, norm(*a, *b), if i<pts.len()-1{","}else{""});
            }
            o.push_str("]}\n");
        }
        "rust" => {
            let _ = writeln!(o, "// Eisenstein disk radius={} ({} points)\npub const DISK: [(i32, i32); {}] = [", radius, pts.len(), pts.len());
            for (a, b) in &pts { let _ = writeln!(o, "    ({}, {}),", a, b); }
            o.push_str("];\n");
        }
        "python" => {
            let _ = writeln!(o, "# Eisenstein disk radius={} ({} points)\nDISK = [", radius, pts.len());
            for (a, b) in &pts { let _ = writeln!(o, "    ({}, {}),", a, b); }
            o.push_str("]\n");
        }
        _ => o.push_str("// Use --lang rust|python|json\n"),
    }
    print!("{}", o);
}

fn gen_table(radius: i64, lang: &str) {
    let pts = disk(radius);
    let mut o = String::new();
    match lang {
        "rust" => {
            let _ = writeln!(o, "use std::collections::HashMap;\n/// Coord→index for {} points\npub fn coord_index() -> HashMap<(i32, i32), usize> {{\n    let mut m = HashMap::with_capacity({});", pts.len(), pts.len());
            for (i, (a, b)) in pts.iter().enumerate() { let _ = writeln!(o, "    m.insert(({}, {}), {});", a, b, i); }
            o.push_str("    m\n}\n");
        }
        "json" => {
            o.push_str("{\"mapping\": {\n");
            for (i, (a, b)) in pts.iter().enumerate() { let _ = writeln!(o, "  \"{},{}\": {}{}", a, b, i, if i<pts.len()-1{","}else{""}); }
            o.push_str("}}\n");
        }
        _ => o.push_str("// Use --lang rust|json\n"),
    }
    print!("{}", o);
}

fn gen_distances(radius: i64, lang: &str) {
    let pts = disk(radius);
    let mut o = String::new();
    match lang {
        "json" => {
            o.push_str("[\n");
            for (i, (a, b)) in pts.iter().enumerate() {
                let hd = a.abs().max(b.abs()).max((a-b).abs());
                let _ = writeln!(o, "  {{\"a\": {}, \"b\": {}, \"hex_dist\": {}}}{}", a, b, hd, if i<pts.len()-1{","}else{""});
            }
            o.push_str("]\n");
        }
        _ => o.push_str("// Use --lang json\n"),
    }
    print!("{}", o);
}

fn verify(radius: i64) {
    let pts = disk(radius);
    let r2 = radius * radius;
    let mut err = 0;
    let pt_set: HashSet<(i64, i64)> = pts.iter().copied().collect();

    for &(a, b) in &pts { if norm(a, b) > r2 { eprintln!("FAIL norm: ({},{})", a, b); err += 1; } }
    let mut seen = HashSet::new();
    for &p in &pts { if !seen.insert(p) { eprintln!("FAIL dup: {:?}", p); err += 1; } }
    for a in -radius..=radius { for b in -radius..=radius { if norm(a,b) <= r2 && !pt_set.contains(&(a,b)) { eprintln!("FAIL missing: ({},{})", a, b); err += 1; } } }
    for &(a, b) in &pts {
        let (mut ra, mut rb) = (a, b);
        for _ in 0..6 { let na = -rb; let nb = ra + rb; ra = na; rb = nb; }
        if ra != a || rb != b { eprintln!("FAIL rot6: ({},{}) → ({},{})", a, b, ra, rb); err += 1; }
    }

    if err == 0 {
        println!("✅ All verifications passed (radius={})", radius);
        println!("   {} points, {} norms, {} rotation checks", pts.len(), pts.len(), pts.len());
    } else { eprintln!("❌ {} errors", err); std::process::exit(1); }
}

fn stats(radius: i64) {
    let pts = disk(radius);
    let unit = pts.iter().filter(|&&(a,b)| norm(a,b) == 1).count();
    let mut rings = BTreeMap::new();
    for &(a, b) in &pts {
        let hd = a.abs().max(b.abs()).max((a-b).abs());
        *rings.entry(hd).or_insert(0) += 1;
    }
    println!("Eisenstein Hex Disk (radius = {})", radius);
    println!("══════════════════════════════════════");
    println!("Points: {} (norm ≤ {})", pts.len(), radius*radius);
    println!("Unit vectors: {} (norm = 1, D6 neighbors of origin)", unit);
    println!("Rings:");
    for (r, c) in &rings { if *r <= 10 || *r == radius { println!("  ring {:3}: {:5} pts", r, c); } else if *r == 11 { println!("  ..."); } }
    println!("Coverage: {:.1}× denser than Z² square", pts.len() as f64 / ((2*radius+1)*(2*radius+1)) as f64);
}
