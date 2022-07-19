for x in $(solana program show --programs | awk 'RP==0 {print $1}'); do 
    if [[ $x != "Program" ]]; 
    then 
        solana program close $x;
    fi
done