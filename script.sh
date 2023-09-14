export LASER="$HOME/Work/uni/clanok_brno/LASER" # path with LASER embeddings -  needed for VECALIGN

fn="$1"

if [ -z "$fn"]
then
	echo "missing argument"
	exit 1
fi

# echo "\n***** BLEUALIGN *****"
# echo "Running BleuAligner..."
# python3 Bleualign-master/bleualign.py --source "texts/${fn}/${fn}_en.txt" --target "texts/${fn}/${fn}_sk.txt" --output texts/${fn}/${fn}_bleualigned --srctotarget texts/${fn}/${fn}_en_googletrans.txt
# echo "\n***** VECALIGN - LASER *****"
# echo "Creating overlap files for source..."
# python3 vecalign/overlap.py -i "texts/${fn}/${fn}_en.txt" -o "texts/${fn}/overlaps_${fn}_en.txt" -n 10
# echo "Creating overlap files for target..."
# python3 vecalign/overlap.py -i "texts/${fn}/${fn}_sk.txt" -o "texts/${fn}/overlaps_${fn}_sk.txt" -n 10
# echo "Creating embedding files for source..."
# bash LASER/tasks/embed//embed.sh texts/${fn}/overlaps_${fn}_en.txt texts/${fn}/overlaps_${fn}_en.emb
# echo "Creating embedding files for target..."
# bash LASER/tasks/embed//embed.sh texts/${fn}/overlaps_${fn}_sk.txt texts/${fn}/overlaps_${fn}_sk.emb
# echo "\n***** VECALIGN *****"
# echo "Running Vecaligner..."
# python3 vecalign/vecalign.py -s texts/${fn}/${fn}_en.txt -t texts/${fn}/${fn}_sk.txt --src_embed texts/${fn}/overlaps_${fn}_en.txt texts/${fn}/overlaps_${fn}_en.emb --tgt_embed texts/${fn}/overlaps_${fn}_sk.txt texts/${fn}/overlaps_${fn}_sk.emb > texts/${fn}/vecaling_result_${fn}.txt
echo "Running vecaling_to_text..."
./vecalign_to_text/vecalign_to_text -s texts/${fn}/${fn}_en.txt -t texts/${fn}/${fn}_sk.txt -a texts/${fn}/vecaling_result_${fn}.txt
echo "\n***** HUNALIGN *****"
echo "Running Hunaligner..."
./hunalign/src/hunalign/hunalign hunalign/data/sk-en.dic texts/${fn}/${fn}_sk.txt texts/${fn}/${fn}_en.txt -text > texts/${fn}/${fn}_hunalign.txt
echo "Running hunaling_to_text..."
./hunalign_to_text/hungaling_to_text --source texts/${fn}/${fn}_hunalign.txt
echo "\n***** BILINGUAL SENTENCE ALIGNER *****"
echo "Running Bilingual sentence aligner..."
./bilingual-sentence-aligner/align-sents-all.pl texts/${fn}/${fn}_en.txt texts/${fn}/${fn}_sk.txt
echo "***** EVALUATION *****"
echo "Running aligner_eval: evaluation of Vecaligner..."
./evaluation/aligner_eval -s texts/${fn}/${fn}_en.txt_vecaligned.txt -t texts/${fn}/${fn}_sk.txt_vecaligned.txt --source_ref texts/${fn}/${fn}_ref_en.txt --target_ref texts/${fn}/${fn}_ref_sk.txt #--verbose
echo "Running aligner_eval: evaluation of BleuAligner..."
./evaluation/aligner_eval -s texts/${fn}/${fn}_bleualigned-s -t texts/${fn}/${fn}_bleualigned-t --source_ref texts/${fn}/${fn}_ref_en.txt --target_ref texts/${fn}/${fn}_ref_sk.txt #--verbose
echo "Running aligner_eval: evaluation of Hunaligner..."
./evaluation/aligner_eval -s texts/${fn}/${fn}_hunalign.txt_hunalign_en.txt -t texts/${fn}/${fn}_hunalign.txt_hunalign_sk.txt --source_ref texts/${fn}/${fn}_ref_en.txt --target_ref texts/${fn}/${fn}_ref_sk.txt #--verbose
echo "Running aligner_eval: evaluation of Bilingual sentence aligner..."
./evaluation/aligner_eval -s texts/${fn}/${fn}_en.txt.aligned -t texts/${fn}/${fn}_sk.txt.aligned --source_ref texts/${fn}/${fn}_ref_en.txt --target_ref texts/${fn}/${fn}_ref_sk.txt #--verbose
