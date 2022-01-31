# Q_rsqrt in Rust
Rewriting the infamous Quake III fast inverse square root function in Rust <br><br>

This is nothing but a completely useless hack today, DO NOT use it in any programs expecting them to run faster. <br>
According to this , just doing `1/sqrt(x)` is around 4-5x faster than Q_rsqrt, and Q_rsqrt is only an *estimation*, so you lose accuracy in the process.<br><br>

However, it was fun to write, and it taught me a little about memory and black magic number manipulation.<br>

## Original C code from Q3A
```c
float Q_rsqrt( float number ) {
	long i;
	float x2, y;
	const float threehalfs = 1.5F;

	x2 = number * 0.5F;
	y  = number;
	i  = * ( long * ) &y;                       // evil floating point bit level hacking
	i  = 0x5f3759df - ( i >> 1 );               // what the fuck? 
	y  = * ( float * ) &i;
	y  = y * ( threehalfs - ( x2 * y * y ) );   // 1st iteration
	// y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed
	
	return y;
}
```
## Rust rewrite of the same function 
```rust
fn q_rsqrt(number: f32) -> f32 {
    let mut i: i32;
    let x2: f32;
    let mut y: f32;
    const THREEHALVES: f32 = 1.5;

    x2 = number * 0.5;
    y = number;

    // Evil floating point bit level hacking
    i = y.to_bits() as i32;

    // What the fuck?
    i = 0x5f3759df - (i >> 1);
    y = f32::from_bits(i as u32);

    // 1st iteration
    y = y * (THREEHALVES - (x2 * y * y));

    // 2nd iteration, this can be removed
    // y = y * (THREEHALVES - (x2 * y * y));

    return y;
}
```

