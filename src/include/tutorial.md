**Deepmath tutorial**

Hello from Deepmath! Deepmath is an alternative implementation of
*"Deep Learning for Symbolic Mathematics"* in Rust. Its neural network
can solve a variety of integration and differentiation problems.

Follow the easy steps below to get started.

**Step 1: Get the dataset**
```
deepmath --prepare --debug
```

**Step 2: Train the model**
```
deepmath --train --debug
```

By default Deepmath will train its model to `$PWD/deepmath_model/model.dat`. You can use the `--train_to <yourmodel.dat>` option instead to train
to the location of your choice.

**Step 3: Use the model to solve**

Running `predict` will cause Deepmath to launch a WebView window with a 
Jupyter-style interface. The UI will then allow you to make test predictions,
assess model accuracy and metrics, and input custom equations to 
integrate/solve with the model.

You can let deepmath automatically find its trained model, typically saved
to `$PWD/deepmath_model/model.dat`:
```
deepmath --predict --debug
```

Or you can manually specify a model path:
```
deepmath --load "model.dat" --predict --debug
```

Either way, you can then play around with the model using the WebView UI.

This should be enough to get you started. For an overview of all the options,
run `deepmath --help`.