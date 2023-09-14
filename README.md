# Automatic Sentence Alignment Evaluation for English-Slovak Language Pair

## Overview

This Git project is dedicated to the evaluation of existing tools for automatic sentence alignment in the English-Slovak language pair. We have assessed four popular tools: VecAlign, HunAlign, BleuAlign, and Bilingual Sentence Aligner. Additionally, we have developed a custom evaluation tool to measure alignment quality against manually aligned reference data.

## Purpose

The primary objective of this project is to evaluate the performance of various automatic sentence alignment tools in the context of English-Slovak text corpora. The assessment aims to provide insights into the strengths and weaknesses of these tools, ultimately aiding researchers and practitioners in choosing the most suitable tool for their specific needs.

## Tools Evaluated

We have assessed the following automatic sentence alignment tools:

1. **VecAlign:** A tool based on vector embeddings for aligning sentences in bilingual corpora.

2. **HunAlign:** A widely-used sentence alignment tool that employs heuristics and statistical methods.

3. **BleuAlign:** A tool that uses BLEU score-based metrics to align sentences.

4. **Bilingual Sentence Aligner:** A tool designed specifically for aligning bilingual sentence pairs.

## Custom Evaluation Tool

In addition to evaluating existing tools, we have developed a custom evaluation tool. This tool enables the measurement of alignment quality against manually aligned reference data. This custom evaluation helps us establish a benchmark for alignment accuracy.

## Evaluation Metrics

The evaluation of these tools is based on various metrics, including:

- **Precision:** The percentage of correctly aligned sentence pairs out of the total aligned pairs.
- **Recall:** The percentage of correctly aligned sentence pairs out of the total manually aligned pairs.
- **F1 Score:** The harmonic mean of precision and recall, providing a balanced measure of alignment quality.

## How to Use

To use this evaluation framework, follow these steps:

1. **Download LASER Embeddings:** Before running the evaluation, you need to download LASER embeddings for the English-Slovak language pair. The data size is substantial, so we haven't included it in the repository. You can obtain the LASER embeddings dataset from the official LASER repository (provide link) or a suitable source.

2. **Clone the Repository:** Clone this Git repository to your local machine.

3 **Edit the script:** Edit the first line of the `script.sh` in the root directory. It should point to the location with LASER data.

4. **Run the Evaluation Script:** Use the provided evaluation script, specifying the number of texts to use as an argument (e.g., `01`, `02`, etc.). This script will execute all the sentence alignment tools (VecAlign, HunAlign, BleuAlign, and Bilingual Sentence Aligner) and then proceed with the evaluation using your data.

   ```bash
   ./script.sh 01


*Note*: The included binaries for the alignment tools are built for Linux. If you are using a different platform, you can find the source code for these tools in this repository. You can build the tools for your specific platform by following the provided build instructions and then overwrite the binaries in the repository.

## Conclusion

This project offers a comprehensive evaluation of automatic sentence alignment tools for the English-Slovak language pair. Researchers and practitioners can use these findings to make informed decisions about which tool best suits their specific alignment needs.

We welcome contributions, feedback, and further improvements to this evaluation framework. Feel free to get involved and help enhance the assessment of sentence alignment tools.
