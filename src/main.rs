use std::{collections::HashSet,fs,io::{self,Write},time::Duration};
#[derive(Debug,Clone)]
struct Process { pid:u32,ppid:u32,name:String,state:String,rss:u64 }
fn parse(pid:u32,text:&str)->Result<Process,String>{
 let value=|key:&str|text.lines().find_map(|line|line.strip_prefix(key)).map(str::trim).unwrap_or("");
 let ppid=value("PPid:").parse::<u32>().map_err(|_|"Missing or invalid PPid")?;
 let name=value("Name:").to_owned();if name.is_empty(){return Err("Missing Name".into())}
 let rss=value("VmRSS:").split_whitespace().next().unwrap_or("0").parse::<u64>().map_err(|_|"Invalid VmRSS")?;
 Ok(Process{pid,ppid,name,state:value("State:").to_owned(),rss})
}
fn collect()->io::Result<Vec<Process>>{
 let mut rows=Vec::new();for entry in fs::read_dir("/proc")?.flatten(){
  if let Ok(pid)=entry.file_name().to_string_lossy().parse::<u32>(){
   if let Ok(text)=fs::read_to_string(entry.path().join("status")){if let Ok(p)=parse(pid,&text){rows.push(p)}}
  }
 }rows.sort_by_key(|p|p.pid);Ok(rows)
}
fn tree(rows:&[Process])->Vec<(usize,&Process)>{
 fn walk<'a>(pid:u32,depth:usize,rows:&'a[Process],seen:&mut HashSet<u32>,out:&mut Vec<(usize,&'a Process)>){
  if !seen.insert(pid){return}if let Some(p)=rows.iter().find(|p|p.pid==pid){out.push((depth,p));for child in rows.iter().filter(|c|c.ppid==pid&&c.pid!=pid){walk(child.pid,depth+1,rows,seen,out)}}
 }
 let ids:HashSet<u32>=rows.iter().map(|p|p.pid).collect();let mut seen=HashSet::new();let mut out=Vec::new();
 for p in rows.iter().filter(|p|p.ppid==0||!ids.contains(&p.ppid)){walk(p.pid,0,rows,&mut seen,&mut out)}
 for p in rows {if !seen.contains(&p.pid){walk(p.pid,0,rows,&mut seen,&mut out)}}out
}
fn safe(value:&str)->String{value.chars().map(|c|if c.is_control(){'?'}else{c}).collect()}
fn main()->Result<(),Box<dyn std::error::Error>>{
 if !cfg!(target_os="linux"){return Err("Linux /proc is required".into())}
 let args:Vec<String>=std::env::args().skip(1).collect();let mut filter=String::new();let mut sort=false;let mut watch=false;let mut i=0;
 while i<args.len(){match args[i].as_str(){"--memory"=>sort=true,"--watch"=>watch=true,"--filter"=>{i+=1;filter=args.get(i).ok_or("--filter needs text")?.to_lowercase()},"--help"=>{println!("Usage: linux-process-explorer [--memory] [--filter TEXT] [--watch]");return Ok(())},_=>return Err(format!("Unknown argument: {}",args[i]).into())};i+=1;}
 loop{
  let mut rows=collect()?;rows.retain(|p|filter.is_empty()||p.name.to_lowercase().contains(&filter)||p.pid.to_string()==filter);
  if watch{print!("\x1b[2J\x1b[H")};println!("{:>7} {:>7} {:>12}  PROCESS / STATE","PID","PPID","RSS KiB");
  if sort{rows.sort_by_key(|p|std::cmp::Reverse(p.rss));for p in &rows{println!("{:>7} {:>7} {:>12}  {} / {}",p.pid,p.ppid,p.rss,safe(&p.name),safe(&p.state));}}
  else{for(depth,p)in tree(&rows){println!("{:>7} {:>7} {:>12}  {}{} / {}",p.pid,p.ppid,p.rss,"  ".repeat(depth.min(50)),safe(&p.name),safe(&p.state));}}
  println!("{} visible processes; inaccessible/exited processes omitted",rows.len());io::stdout().flush()?;
  if !watch{break}std::thread::sleep(Duration::from_secs(2));
 }Ok(())
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn parses_status(){let p=parse(8,"Name:\tworker\nState:\tS (sleeping)\nPPid:\t2\nVmRSS:\t512 kB\n").unwrap();assert_eq!((p.pid,p.ppid,p.rss),(8,2,512));assert!(parse(1,"Name: broken").is_err());}
 #[test]fn keeps_orphans_and_breaks_cycles(){let rows=vec![Process{pid:1,ppid:2,name:"a".into(),state:"S".into(),rss:0},Process{pid:2,ppid:1,name:"b".into(),state:"S".into(),rss:0},Process{pid:3,ppid:99,name:"orphan".into(),state:"S".into(),rss:0}];let result=tree(&rows);assert_eq!(result.len(),3);assert_eq!(result[0].1.pid,3);}
}
