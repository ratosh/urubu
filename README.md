# Urubu

A chess engine written in Rust using neural networks.

## Network

- Square as a number from 0 (A1) to 63 (H8);
- Piece as a number from 1 (pawn) to 6 (king);
- Color as a number from 0 (white) to 1 (black);

### Input 1 (768 inputs)
Similar to a simple bitboard board representation (https://www.chessprogramming.org/Bitboard_Board-Definition).

We can represent the board as 768 inputs: (square number * (piece number * (color + 1)))

### Input 2 (512 inputs)
Similar to a dense bitboard representation.
- First two layers would be colors.
- Other layers would be pieces (6 layers).

Each piece would show up on 2 different layers (In a color layer and in a piece layer).

(square number * (piece number * (color + 1)))

### Output

- Win chance from white side perspective?
- Convert to i8 value?

### Layers

- Number of hidden layers;
- Number of nodes per layer;

### File
- Layer information (Same as in configuration);
- Weights;

## Training

- Written in Python;
- Load dataset;
- Configurable;
- Actual training;

### Training set

- Using PGN files;
- Extract different positions;
- Use game result to guide NN output;

### Training data

- CCRL: http://www.computerchess.org.uk/ccrl/4040/games.html
- Lichess: https://database.lichess.org/
- Lc0? https://storage.lczero.org/files/training_data/
- Stockfish?
- Pirarucu?

### Training configuration:

- Layer information;
- Training batch size;
- Training iterations;
- Learning rate (pref variable and depending on iterations);

## Running
- Engine should be able to load NN file;
- Engine should use the NN to evaluate the position;

## Validation
Similar to fishtest, should run games and check which one performs better. Use existing tools?
- Fishtest
- Openbench

## Automation
Generating and validating the NN should be an automated process, user input should be as simple as possible.

Current idea:
- Generate N networks (in generations);
- Test in a tournament against each other (similar to validation);
- Include test games as training data for the next network generation;

To be tested:
- Use top X networks as starting point for next generation?
- Include our best network into the test pool?

## To discuss

### Aggregate result
- Aggregate position result from different games (having win chance based on many games instead of just one);