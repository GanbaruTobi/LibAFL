//! The testcase is a struct embedded in each corpus.
//! It will contain a respective input, and metadata.

use alloc::string::String;
use core::{convert::Into, default::Default, option::Option, time::Duration};
use serde::{Deserialize, Serialize};

use crate::{
    bolts::serdeany::{SerdeAny, SerdeAnyMap},
    inputs::Input,
    Error,
};

/// An entry in the Testcase Corpus
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "I: serde::de::DeserializeOwned")]
pub struct Testcase<I>
where
    I: Input,
{
    /// The input of this testcase
    input: Option<I>,
    /// Filename, if this testcase is backed by a file in the filesystem
    filename: Option<String>,
    /// Accumulated fitness from all the feedbacks
    fitness: u32,
    /// Map of metadatas associated with this testcase
    metadatas: SerdeAnyMap,
    /// Time needed to execute the input
    exec_time: Option<Duration>,
}

/// Impl of a testcase
impl<I> Testcase<I>
where
    I: Input,
{
    /// Returns this testcase with a loaded input
    pub fn load_input(&mut self) -> Result<&I, Error> {
        if self.input.is_none() {
            self.input = Some(I::from_file(self.filename.as_ref().unwrap())?);
        }
        Ok(self.input.as_ref().unwrap())
    }

    /// Store the input to disk if possible
    pub fn store_input(&mut self) -> Result<bool, Error> {
        let fname;
        match self.filename() {
            Some(f) => {
                fname = f.clone();
            }
            None => {
                return Ok(false);
            }
        };
        match self.input_mut() {
            None => Ok(false),
            Some(i) => {
                i.to_file(fname)?;
                Ok(true)
            }
        }
    }

    /// Get the input, if any
    #[inline]
    pub fn input(&self) -> &Option<I> {
        &self.input
    }

    /// Get the input, if any (mutable)
    #[inline]
    pub fn input_mut(&mut self) -> &mut Option<I> {
        &mut self.input
    }

    /// Set the input
    #[inline]
    pub fn set_input(&mut self, input: I) {
        self.input = Some(input);
    }

    /// Get the filename, if any
    #[inline]
    pub fn filename(&self) -> &Option<String> {
        &self.filename
    }

    /// Get the filename, if any (mutable)
    #[inline]
    pub fn filename_mut(&mut self) -> &mut Option<String> {
        &mut self.filename
    }

    /// Set the filename
    #[inline]
    pub fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }

    /// Get the fitness
    #[inline]
    pub fn fitness(&self) -> u32 {
        self.fitness
    }

    /// Get the fitness (mutable)
    #[inline]
    pub fn fitness_mut(&mut self) -> &mut u32 {
        &mut self.fitness
    }

    /// Set the fitness
    #[inline]
    pub fn set_fitness(&mut self, fitness: u32) {
        self.fitness = fitness;
    }

    /// Get all the metadatas into an HashMap (mutable)
    #[inline]
    pub fn metadatas(&mut self) -> &mut SerdeAnyMap {
        &mut self.metadatas
    }

    /// Add a metadata
    #[inline]
    pub fn add_metadata<M>(&mut self, meta: M)
    where
        M: SerdeAny,
    {
        self.metadatas.insert(meta);
    }

    /// Create a new Testcase instace given an input
    #[inline]
    pub fn new<T>(input: T) -> Self
    where
        T: Into<I>,
    {
        Testcase {
            input: Some(input.into()),
            filename: None,
            fitness: 0,
            metadatas: SerdeAnyMap::new(),
            exec_time: None,
        }
    }

    /// Create a new Testcase instace given an input and a filename
    #[inline]
    pub fn with_filename(input: I, filename: String) -> Self {
        Testcase {
            input: Some(input),
            filename: Some(filename),
            fitness: 0,
            metadatas: SerdeAnyMap::new(),
            exec_time: None,
        }
    }

    #[inline]
    pub fn default() -> Self {
        Testcase {
            input: None,
            filename: None,
            fitness: 0,
            metadatas: SerdeAnyMap::new(),
            exec_time: None,
        }
    }
}

/// Create a testcase from an input
impl<I> From<I> for Testcase<I>
where
    I: Input,
{
    fn from(input: I) -> Self {
        Testcase::new(input)
    }
}
