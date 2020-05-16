//! This is quite advanced example.
//! As example, we will implement simple `xargs` clone.
//! `xargs` accepts arguments template as `argv` and
//! several lines on stdin. For each line, `xargs` interpolates
//! template (changing "{}" to line) and executes this command.
//! Since this is only example, we will not achieve any compativility,
//! advanced features etc.
//! However, we still don't care for structured output here, again
//! for simplicity.

use clap::{App, Arg};
use climsg::{show, ScopedVisitor};
use std::io::BufRead as _;

const DUMMY: &dyn erased_serde::Serialize = &();

fn main() {
    let matches = App::new("climsg-xargs")
        .arg(Arg::with_name("template").last(true).multiple(true))
        .arg(Arg::with_name("parallel").short("p"))
        .get_matches();

    // dbg!(&matches);

    let args_template: Vec<_> = matches.values_of("template").unwrap().collect();

    let vis = climsg::TextVisitor::new();

    let stdin_lines = std::io::BufReader::new(std::io::stdin()).lines();

    // in this Vec we will put all commands that must be executed
    let mut jobs = Vec::new();

    let mut counter = 0..;

    for line in stdin_lines {
        let line = line.expect("io error");
        let job_id = format!("child-{}", counter.next().unwrap());
        jobs.push((interpolate_args(&args_template, &line), job_id))
    }

    // now we will run this jobs

    if matches.value_of("parallel").is_some() {
        todo!()
    } else {
        for job in jobs {
            let vis = vis.scoped("job");
            show!(&vis, DUMMY, "will execute: {:?}", &job.0,);
            execute_job(&job.0, &vis.scoped("output"))
        }
    }
}

/// This functions finds all occurences of "{}" and replaces them with `line`
fn interpolate_args(template: &[&str], line: &str) -> Vec<String> {
    let mut v = Vec::new();
    for s in template {
        let new_s = s.replace("{}", line);
        v.push(new_s);
    }
    v
}

fn execute_job(args: &[String], vis: &dyn climsg::Visitor) {
    if let Err(err) = inner_execute_job(args, vis) {
        show!(vis, DUMMY, "error: {}", err,);
    }
}

fn inner_execute_job(args: &[String], vis: &dyn climsg::Visitor) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new(&args[0]);
    cmd.args(&args[1..]);
    cmd.stdin(std::process::Stdio::null());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn()?;
    let mut stdout = std::io::BufReader::new(child.stdout.take().unwrap());
    let mut stderr = std::io::BufReader::new(child.stderr.take().unwrap());
    let mut line = String::new();
    loop {
        line.clear();
        stdout.read_line(&mut line)?;
        if line.is_empty() {
            stderr.read_line(&mut line)?;
        }
        if line.is_empty() {
            // both stdout and stderr are exhausted now
            break;
        }
        show!(vis, DUMMY, "child: {}", line.trim(),);
    }
    Ok(())
}
