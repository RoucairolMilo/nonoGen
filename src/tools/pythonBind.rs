use std::io::stderr;
use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};

/*

pub fn from_dir(export_dir: &str) -> Result<PyModule, &'static str> {
    Python::with_gil(|py| {
        return PyModule::from_code(
                py,
                r#"
import sys
import math
import random
import numpy as np
import random as pr
from rdkit import Chem
# from make_smile import *
# from keras.models import load_model
# from keras.preprocessing import sequence
import tensorflow as tf
from tensorflow.keras.preprocessing import sequence

val = ['\n', '&', 'C', '(', ')', 'c', '1', '2', 'o', '=', 'O', 'N', '3', 'F',
       '[C@@H]', 'n', '-', '#', 'S', 'Cl', '[O-]', '[C@H]', '[NH+]', '[C@]',
       's', 'Br', '/', '[nH]', '[NH3+]', '4', '[NH2+]', '[C@@]', '[N+]',
       '[nH+]', '\\', '[S@]', '5', '[N-]', '[n+]', '[S@@]', '[S-]', '6', '7',
       'I', '[n-]', 'P', '[OH+]', '[NH-]', '[P@@H]', '[P@@]', '[PH2]',
       '[P@]', '[P+]', '[S+]', '[o+]', '[CH2-]', '[CH-]', '[SH+]', '[O+]',
       '[s+]', '[PH+]', '[PH]', '8', '[S@@+]']


def load_model(model_dir):
    global loaded_model
    # Load the model from the saved file
    loaded_model = tf.keras.models.load_model(model_dir)
    print("Loaded model: {}".format(model_dir))
    return loaded_model


def predict(incompleteSMILES):
    # list/tuple of strings turend to length 81
    state = [i for i in incompleteSMILES]
    max_len = 81
    get_int = [val.index(state[j]) for j in range(len(state))]
    # print("get_int", get_int)
    x = np.reshape(get_int, (1, len(get_int)))
    # print("x", x)
    x_pad = sequence.pad_sequences(x, maxlen=max_len, dtype='int32', padding='post', truncating='pre', value=0.0)
    # print("x_pad", x_pad)
    predictions = loaded_model.predict(x_pad)
    preds = predictions[0][len(get_int) - 1]

    return preds

load_model("Neural\SMILES")
    "#,
                "readNN.py",
                "readNN",
            ).unwrap();

        });

    return Err("failed to get a python interpretor")
    }

pub fn predict(incompleteSMILES : Vec<String>, loadedNN : PyModule) -> Vec<f64> {
    Python::with_gil(|py| {
    let pred = loadedNN.getattr("predict")?.call1((incompleteSMILES))?.extract()?;
    println!("ok");
    return pred})
}

 */

