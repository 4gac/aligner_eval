#!/usr/bin/perl

# (c) Microsoft Corporation. All rights reserved.

($sent_file_1,$sent_file_2,$threshold) = @ARGV;

if (!defined($threshold)) {
    $threshold = .50;
}

print "\nFinding length-based alignment\n";
system("perl bilingual-sentence-aligner/align-sents-dp-beam7.pl $sent_file_1 $sent_file_2");
print "\n========================================================\n";
print "\nFiltering initial high-probability aligned sentences\n";
system("perl bilingual-sentence-aligner/filter-initial-aligned-sents.pl $sent_file_1 $sent_file_2");
print "\n========================================================\n";
print "\nBuilding word association model\n";
system("perl bilingual-sentence-aligner/build-model-one6.pl $sent_file_1 $sent_file_2");
print "\n========================================================\n";
print "\nFinding alignment based on word associations and lengths\n";
system("perl bilingual-sentence-aligner/align-sents-length-plus-words3.pl $sent_file_1 $sent_file_2");
print "\n========================================================\n";
print "\nFiltering final high-probability aligned sentences\n";
system("perl bilingual-sentence-aligner/filter-final-aligned-sents.pl $sent_file_1 $sent_file_2 $threshold");
print "\n";
