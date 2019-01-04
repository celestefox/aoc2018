use chrono::{DateTime, TimeZone, Timelike, Utc};
use nom::types::CompleteStr;
use nom::{alt, call, do_parse, error_position, map_res, named, tag, take_while, value};
use std::collections::HashMap;
use std::str::FromStr;

/* examples:
 * [1518-11-01 23:58] Guard #99 begins shift
 * [1518-11-02 00:40] falls asleep
 * [1518-11-02 00:50] wakes up
 */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GuardEvent {
    NewGuard(u32),
    GuardSleeping,
    GuardAwake,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GuardStatusUpdate {
    date: DateTime<Utc>,
    event: GuardEvent,
}

fn is_digit(s: char) -> bool {
    "0123456789".contains(s)
}

named!(simple_num<CompleteStr, u32>, map_res!(take_while!(is_digit), |s: CompleteStr| u32::from_str(*s)));
named!(simple_signed_num<CompleteStr, i32>, map_res!(take_while!(is_digit), |s: CompleteStr| i32::from_str(*s)));

named!(
    guard_action_date <CompleteStr, DateTime<Utc>>,
    do_parse!(
        tag!("[")
            >> year: simple_signed_num
            >> tag!("-")
            >> month: simple_num
            >> tag!("-")
            >> day: simple_num
            >> tag!(" ")
            >> hour: simple_num
            >> tag!(":")
            >> minute: simple_num
            >> tag!("]")
            >> (Utc.ymd(year, month, day).and_hms(hour, minute, 0))
    )
);

named!(guard_event<CompleteStr, GuardEvent>, alt!(value!(GuardEvent::GuardSleeping, tag!("falls asleep")) | value!(GuardEvent::GuardAwake, tag!("wakes up")) | do_parse!(tag!("Guard #") >> id: simple_num >> tag!(" begins shift") >> (GuardEvent::NewGuard(id)))));

named!(guard_update<CompleteStr, GuardStatusUpdate>, do_parse!(date: guard_action_date >> tag!(" ") >> event: guard_event >> (GuardStatusUpdate{ date, event })));

fn process_log(entries: Vec<&str>) -> Vec<(u32, u32, u32)> {
    let mut result = Vec::new();
    let mut current_guard: u32 = 0;
    let mut start_time: u32 = 0;
    for entry in entries {
        let parsed: GuardStatusUpdate = guard_update(CompleteStr(entry)).unwrap().1;
        match parsed.event {
            GuardEvent::NewGuard(guard_id) => current_guard = guard_id,
            GuardEvent::GuardSleeping => start_time = parsed.date.minute(),
            GuardEvent::GuardAwake => {
                result.push((current_guard, start_time, parsed.date.minute()))
            }
        }
    }
    result
}

fn sum_sleeps(sleeps: Vec<(u32, u32, u32)>) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    for sleep in sleeps.iter() {
        *result.entry(sleep.0).or_insert(0) += sleep.2 - sleep.1;
    }
    result
}

fn most_slept_minute(sleeps: Vec<(u32, u32, u32)>, id: &u32) -> u32 {
    let relevant_sleeps: Vec<&(u32, u32, u32)> =
        sleeps.iter().filter(|sleep| sleep.0 == *id).collect();
    //println!("{:?}", relevant_sleeps);
    let mut minutes: HashMap<u32, u32> = HashMap::new();
    for sleep in relevant_sleeps.iter() {
        for minute in sleep.1..sleep.2 {
            *minutes.entry(minute).or_insert(0) += 1;
        }
    }
    //println!("{:?}", minutes);
    *minutes.iter().max_by_key(|&v| v.1).unwrap().0
}

fn total_up_minutes(sleeps: Vec<(u32, u32)>) -> HashMap<u32, u32> {
    let mut minutes: HashMap<u32, u32> = HashMap::new();
    for sleep in sleeps.iter() {
        for minute in sleep.0..sleep.1 {
            *minutes.entry(minute).or_insert(0) += 1;
        }
    }
    minutes
}

pub fn main() {
    let file = include_str!("../../inputs/day4");
    // Okay, different approach
    let mut lines: Vec<&str> = file.lines().collect::<Vec<&str>>();
    lines.sort();
    let sleeps = process_log(lines);
    let totals = sum_sleeps(sleeps.clone());
    let most_tired = totals.iter().max_by_key(|total| total.1).unwrap();
    let minute = most_slept_minute(sleeps.clone(), most_tired.0);
    println!(
        "The most tired guard: {:?}, minute is {}, answer is {}",
        most_tired,
        minute,
        most_tired.0 * minute
    );
    let guard_totals: HashMap<u32, Vec<(u32, u32)>> = totals
        .iter()
        .map(|guard_total| guard_total.0)
        .map(|&guard| {
            (
                guard,
                sleeps
                    .iter()
                    .filter(|&sleep| sleep.0 == guard)
                    .map(|&sleep| (sleep.1, sleep.2))
                    .collect(),
            )
        })
        .collect();
    let guard_totals: HashMap<u32, HashMap<u32, u32>> = guard_totals
        .iter()
        .map(|(guard, sleeps)| (*guard, total_up_minutes(sleeps.to_vec())))
        .collect();
    let guard_who_was_asleep_at_the_same_time_the_most: u32 = guard_totals
        .iter()
        .map(|(guard, minutes)| (*guard, minutes.iter().max_by_key(|p| p.1).unwrap().1))
        .max_by_key(|p| p.1)
        .unwrap()
        .0;
    let said_guards_minute_most_asleep = guard_totals
        .get(&guard_who_was_asleep_at_the_same_time_the_most)
        .unwrap()
        .iter()
        .max_by_key(|p| p.1)
        .unwrap()
        .0;
    println!("The guard who was asleep the same minute the most is {}, and the minute is {}, so an answer of {}", guard_who_was_asleep_at_the_same_time_the_most, said_guards_minute_most_asleep, guard_who_was_asleep_at_the_same_time_the_most*said_guards_minute_most_asleep);
}
