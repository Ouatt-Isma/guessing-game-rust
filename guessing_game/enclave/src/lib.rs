// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "guessinggame"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;

extern crate sgx_rand as rand;

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;




static mut NUMBER_TO_GUESS: i16 =-1;
static mut MAXNUM: i16 = -1; 
static mut CPT: i16 = 0; 



use sgx_types::*;
use rand::{thread_rng, Rng}; 
// use std::process::exit; 

fn generate_integer() -> i16{

        let mut rng = thread_rng(); 
	let res: u8 = rng.gen();
	
	return res.into(); 
	// return 2; 
}

#[no_mangle]
//Function to get the max value of tentative from the host
pub extern "C" fn enc_get_maxnum(res: i16) -> sgx_status_t{
unsafe{
	
	MAXNUM = res; 
}	
	 sgx_status_t::SGX_SUCCESS 

}


#[no_mangle]
//Function to compare the entered value and the generated random value
pub extern "C" fn enc_compare(a: i16)-> bool{
	let res: bool;
	unsafe{
	CPT+=1; 

	if NUMBER_TO_GUESS==-1 {
		NUMBER_TO_GUESS=generate_integer();


	}
	if MAXNUM<CPT {
		
		 panic!("Enclave autolock because {}(max value)<{}(number of tentative)", MAXNUM, CPT);
		 //panic_abort::sgx_abort();
		 // exit(1);
		// oe_abort(); 
	         
	}
	
	if a==NUMBER_TO_GUESS {
		res = true;
	}
	else {
		res = false; 	
	}

	}

	// sgx_status_t::SGX_SUCCESS
	return res;

}
