#!/bin/bash
#
# > make a file executable
# chmod +x ./_init.sh
#
# > positional arguments:
# 1st - Stash
# 2nd - Identity
# 3rd - Queued session keys (0x..)
# 4th - Is active? (true/false)
# 5th - Session keys queued? (true/false)
# 6th - Era
# 7th - Session
# 8th - Eras session index [1,2,3,4,5,6]
# 9th - Current block
#
# The following arguments depend on exposed flags
# 10th - Network name (--expose-network flag must be set)
# 11th - Network token symbol (--expose-network flag must be set)
# 12th - Network token decimals (--expose-network flag must be set)
#
# 13th - Validator Total stake (--expose-nominators flag must be set)
# 14th - Validator Own stake (--expose-nominators flag must be set)
# 15th - Active Nominators stashes [stash_1, stash_2, ..] (--expose-nominators flag must be set)
# 16th - Active Nominators stakes [stake_1, stake_2, ..] (--expose-nominators flag must be set)
#
# 17th - Number of Authored blocks (Current Session) (--expose-authored-blocks flag must be set)
# 18th - Not applicable
#
# 19th - All Nominators stashes [stash_1, stash_2, ..] (--expose-all-nominators flag must be set)
#
# > Special character '!' controls message visibility on Matrix (Element)
# Any message that starts with '!' will be sent to Matrix, to the user private room
#
# echo "! This message will be sent to Matrix"
# echo "This message will NOT be sent to Matrix"
#
# ***** START *****
#
echo #" e.g. Write your own script here"
echo "--------------------------------"
echo "Positional arguments:"
echo "1st - Stash -> $1"
echo "2nd - Identity -> $2"
echo "3rd - Queued session keys -> ${3:0:16}.."
echo "4th - Is active? -> $4"
echo "5th - Session keys queued? -> $5"
echo "6th - Era -> $6"
echo "7th - Session -> $7"
echo "8th - Eras session index -> $8"
echo "9th - Current block -> $9"
echo "(10th) - Network name -> ${10}"
echo "(11th) - Network token symbol -> ${11}"
echo "(12th) - Network token decimals -> ${12}"
echo "(13th) - Projected APR -> ${13}"
echo "(14th) - Validator total stake -> ${14}"
echo "(15th) - Validator own stake -> ${15}"
echo "(16th) - Active Nominators -> ${16}"
echo "(17th) - Active Nominators Stake -> ${17}"
echo "(18th) - Number of Authored blocks in current Session -> ${18}"
echo "(19th) - Number of Authored blocks in previous 6 Sessions -> ${19}"
echo "(20th) - All Nominators -> ${20}"
echo "(21th) - NA"
echo "(22th) - Is Para Validator? -> ${22}"
echo "(23th) - Number of Para Validator times in previous 6 Sessions -> ${23}"
echo "(24th) - Last era validator points -> ${24}"
echo "(25th) - Last era avg validators points -> ${25}"
# Leave room to stay compatible with future turboflakes additions...
echo "(36th) - Current era validator points -> ${36}"
echo "-------------------------------"
if [ "$4" = "true" ]
then
  echo " 🟢 -> 😎"
else
  echo " 🔴 -> 😤"
fi
#
# ***** END *****
