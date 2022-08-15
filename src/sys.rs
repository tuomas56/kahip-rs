#![allow(non_upper_case_globals)]

pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const FAST: ::std::os::raw::c_int = 0;
pub const ECO: ::std::os::raw::c_int = 1;
pub const STRONG: ::std::os::raw::c_int = 2;
pub const FASTSOCIAL: ::std::os::raw::c_int = 3;
pub const ECOSOCIAL: ::std::os::raw::c_int = 4;
pub const STRONGSOCIAL: ::std::os::raw::c_int = 5;
pub const MAPMODE_MULTISECTION: ::std::os::raw::c_int = 0;
pub const MAPMODE_BISECTION: ::std::os::raw::c_int = 1;
extern "C" {
    pub fn kaffpa(
        n: *mut ::std::os::raw::c_int,
        vwgt: *mut ::std::os::raw::c_int,
        xadj: *mut ::std::os::raw::c_int,
        adjcwgt: *mut ::std::os::raw::c_int,
        adjncy: *mut ::std::os::raw::c_int,
        nparts: *mut ::std::os::raw::c_int,
        imbalance: *mut f64,
        suppress_output: bool,
        seed: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        edgecut: *mut ::std::os::raw::c_int,
        part: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn kaffpa_balance(
        n: *mut ::std::os::raw::c_int,
        vwgt: *mut ::std::os::raw::c_int,
        xadj: *mut ::std::os::raw::c_int,
        adjcwgt: *mut ::std::os::raw::c_int,
        adjncy: *mut ::std::os::raw::c_int,
        nparts: *mut ::std::os::raw::c_int,
        imbalance: *mut f64,
        perfectly_balance: bool,
        suppress_output: bool,
        seed: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        edgecut: *mut ::std::os::raw::c_int,
        part: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn kaffpa_balance_NE(
        n: *mut ::std::os::raw::c_int,
        vwgt: *mut ::std::os::raw::c_int,
        xadj: *mut ::std::os::raw::c_int,
        adjcwgt: *mut ::std::os::raw::c_int,
        adjncy: *mut ::std::os::raw::c_int,
        nparts: *mut ::std::os::raw::c_int,
        imbalance: *mut f64,
        suppress_output: bool,
        seed: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        edgecut: *mut ::std::os::raw::c_int,
        part: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn process_mapping(
        n: *mut ::std::os::raw::c_int,
        vwgt: *mut ::std::os::raw::c_int,
        xadj: *mut ::std::os::raw::c_int,
        adjcwgt: *mut ::std::os::raw::c_int,
        adjncy: *mut ::std::os::raw::c_int,
        hierarchy_parameter: *mut ::std::os::raw::c_int,
        distance_parameter: *mut ::std::os::raw::c_int,
        hierarchy_depth: ::std::os::raw::c_int,
        mode_partitioning: ::std::os::raw::c_int,
        mode_mapping: ::std::os::raw::c_int,
        imbalance: *mut f64,
        suppress_output: bool,
        seed: ::std::os::raw::c_int,
        edgecut: *mut ::std::os::raw::c_int,
        qap: *mut ::std::os::raw::c_int,
        part: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn node_separator(
        n: *mut ::std::os::raw::c_int,
        vwgt: *mut ::std::os::raw::c_int,
        xadj: *mut ::std::os::raw::c_int,
        adjcwgt: *mut ::std::os::raw::c_int,
        adjncy: *mut ::std::os::raw::c_int,
        nparts: *mut ::std::os::raw::c_int,
        imbalance: *mut f64,
        suppress_output: bool,
        seed: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        num_separator_vertices: *mut ::std::os::raw::c_int,
        separator: *mut *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn reduced_nd(
        n: *mut ::std::os::raw::c_int,
        xadj: *mut ::std::os::raw::c_int,
        adjncy: *mut ::std::os::raw::c_int,
        suppress_output: bool,
        seed: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        ordering: *mut ::std::os::raw::c_int,
    );
}
