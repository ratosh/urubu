# Urubu

A chess engine written in Rust using neural networks.

## Dataset

### NN Input
TODO

### NN Output

- Win chance from white side perspective?
- Convert to i8 value?

### Training position

- Using PGN files;
- Extract different positions;
- Use game result to guide NN output;

### Training data

- CCRL: http://www.computerchess.org.uk/ccrl/4040/games.html
- Lichess: https://database.lichess.org/
- Lc0? https://storage.lczero.org/files/training_data/
- Stockfish?
- Pirarucu?

## Training

- Written in Python;
- Load dataset;
- Configurable;
- Actual training;

### Training configuration:

- Number of hidden layers;
- Number of nodes per layer (can it be variable?);
- Training batch size;
- Training iterations;
- Learning rate (pref variable and depending on iterations);

## NN file
- Layer information (Same as in configuration);
- Weights;

## Running
- Engine should be able to load NN file;
- Engine should use the NN to evaluate the position;

## Validação
Similar to fishtest, should run games and check which one performs better. Use existing tools?
- Fishtest
- Openbench

## Automation
Generating and validating the NN should be an automated process, user input should be as simple as possible.
- Automate checking for different NN configuration?

## To discuss

### Aggregate result
- Aggregate position result from different games (having win chance based on many games instead of just one);