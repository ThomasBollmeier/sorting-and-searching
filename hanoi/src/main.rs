use clap::Parser;

const NUM_POSTS: usize = 3;

#[derive(Debug, Parser)]
struct Options {
    #[arg(short, long, default_value = "3", help = "Number of disks to move")]
    num_disks: usize,
}

fn draw_posts(posts: &[Vec<usize>; NUM_POSTS]) {
    let num_disks = posts[0].len();
    for r in 0..num_disks {
        for post_idx in 0..3 {
            print!("{:2} ", posts[post_idx][r]);
        }
        println!();
    }
    println!("---------");
}

// Move one disk from from_post to to_post.
fn move_disk(posts: &mut [Vec<usize>; NUM_POSTS], from_post: usize, to_post: usize) {
    let num_disks = posts[0].len();
    let mut from_idx: Option<usize> = None;
    for r in 0..num_disks {
        if posts[from_post][r] != 0 {
            from_idx = Some(r);
            break;
        }
    }
    if from_idx.is_none() {
        return;
    }
    let from_idx = from_idx.unwrap();

    let mut to_idx: Option<usize> = None;
    for r in (0..num_disks).rev() {
        if posts[to_post][r] == 0 {
            to_idx = Some(r);
            break;
        }
    }
    if to_idx.is_none() {
        return;
    }
    let to_idx = to_idx.unwrap();

    posts[to_post][to_idx] = posts[from_post][from_idx];
    posts[from_post][from_idx] = 0;

    draw_posts(posts);
}

fn move_n_disks(posts: &mut [Vec<usize>; NUM_POSTS], n: usize, from: usize, to: usize, via: usize) {
    match n {
        0 => return,
        1 => {
            move_disk(posts, from, to);
        }
        _ => {
            move_n_disks(posts, n - 1, from, via, to);
            move_disk(posts, from, to);
            move_n_disks(posts, n - 1, via, to, from);
        }
    }
}

fn move_disks(posts: &mut [Vec<usize>; NUM_POSTS], from: usize, to: usize, via: usize) {
    let n = posts[from].len();
    move_n_disks(posts, n, from, to, via);
}

fn main() {
    let options = Options::parse();
    // Make three posts with NUM_DISKS entries, all set to 0.
    let mut posts = [
        vec![0usize; options.num_disks],
        vec![0usize; options.num_disks],
        vec![0usize; options.num_disks],
    ];

    // Put the disks on the first post in order, smallest first (on top).
    for i in 0..options.num_disks {
        posts[0][i] = i + 1;
    }

    // Draw the initial setup.
    draw_posts(&posts);

    // Move the disks.
    move_disks(&mut posts, 0, 1, 2);
    println!("Ok");
}
