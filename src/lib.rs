pub mod sys;

pub enum Mode {
    Fast = sys::FAST as isize,
    Eco = sys::ECO as isize,
    Strong = sys::STRONG as isize,
    FastSocial = sys::FASTSOCIAL as isize,
    EcoSocial = sys::ECOSOCIAL as isize,
    StrongSocial = sys::STRONGSOCIAL as isize
}

#[derive(Clone, Debug)]
pub struct Graph {
    num_vertices: std::os::raw::c_int,
    adjacency_indices: Vec<std::os::raw::c_int>,
    adjacency_data: Vec<std::os::raw::c_int>,
    vertex_weights: Option<Vec<std::os::raw::c_int>>,
    edge_weights: Option<Vec<std::os::raw::c_int>>
}

impl Graph {
    pub fn new(
        vertices: usize, 
        edges: impl IntoIterator<Item = (usize, usize)>
    ) -> Graph {
        let mut edgelist = vec![Vec::new(); vertices];
        for (a, b) in edges {
            edgelist[a].push(b);
            edgelist[b].push(a);
        }

        let mut adjacency_indices = vec![0];
        let mut adjacency_data = Vec::new();
        for edges in edgelist {
            for end in edges {
                adjacency_data.push(end as std::os::raw::c_int);
            }

            adjacency_indices.push(adjacency_data.len() as std::os::raw::c_int);
        }

        Graph {
            num_vertices: vertices as std::os::raw::c_int,
            adjacency_indices, adjacency_data,
            vertex_weights: None,
            edge_weights: None
        }
    }

    pub fn with_vertex_weights(mut self, weights: impl IntoIterator<Item = usize>) -> Graph {
        self.vertex_weights = Some(weights.into_iter()
            .map(|w| w as std::os::raw::c_int)
            .collect());
        self
    }

    pub fn with_edge_weights(mut self, weights: impl IntoIterator<Item = (usize, usize, usize)>) -> Graph {
        let mut edge_weights = vec![1; self.adjacency_data.len()];
        for (a, b, w) in weights {
            let start = self.adjacency_indices[a] as usize;
            let end = self.adjacency_indices[a + 1] as usize;
            let idx = self.adjacency_data[start..end]
                .iter()
                .position(|&v| v as usize == b)
                .unwrap();
            
            edge_weights[start + idx] = w as std::os::raw::c_int;

            let start = self.adjacency_indices[b] as usize;
            let end = self.adjacency_indices[b + 1] as usize;
            let idx = self.adjacency_data[start..end]
                .iter()
                .position(|&v| v as usize == a)
                .unwrap();
            
            edge_weights[start + idx] = w as std::os::raw::c_int;
        }
        self.edge_weights = Some(edge_weights);

        self
    }

    pub fn node_separator(&self, 
        parts: usize, 
        imbalance: f64, 
        mode: Mode, 
        seed: usize, 
        quiet: bool
    ) -> Vec<usize> {
        let parts = parts as std::os::raw::c_int;
        let mode = mode as std::os::raw::c_int;
        let seed = seed as std::os::raw::c_int;
        let mut num_separator_vertices = 0;
        let mut separator = std::ptr::null_mut::<std::os::raw::c_int>();

        let out_slice = unsafe {
            sys::node_separator(
                &self.num_vertices as *const _ as *mut _, 
                self.vertex_weights.as_ref().map(|v| v.as_ptr() as *mut _)
                    .unwrap_or(std::ptr::null_mut()),
                self.adjacency_indices.as_ptr() as *mut _,
                self.edge_weights.as_ref().map(|v| v.as_ptr() as *mut _)
                    .unwrap_or(std::ptr::null_mut()), 
                self.adjacency_data.as_ptr() as *mut _, 
                &parts as *const _ as *mut _, 
                &imbalance as *const _ as *mut _, 
                quiet, 
                seed, 
                mode, 
                &mut num_separator_vertices as *mut _, 
                &mut separator as *mut _
            );

            std::slice::from_raw_parts(separator, num_separator_vertices as usize)
        };

        out_slice.iter()
            .map(|&v| v as usize)
            .collect::<Vec<_>>()
    }
}