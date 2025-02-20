// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract Groth16Verifier {
    // Scalar field size
    uint256 constant r = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1 = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2 = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1 = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2 = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 16734622443276436061297076243556417593319180165222914383170788472399745600311;
    uint256 constant deltax2 = 6297020007773533610396105937409277179251661377169826757809411476582008402245;
    uint256 constant deltay1 = 11871581632938111492277713806142634543318690094908678346136852881670620231931;
    uint256 constant deltay2 = 15646072706027291242878718792849805700004265720655475432200974562742033916987;

    uint256 constant IC0x = 3537645242598046070061282910025909735157250522564418786968642914181564705820;
    uint256 constant IC0y = 19993750840576381695758930022790649790059243407977041967289013757430497671535;

    uint256 constant IC1x = 16329889066742244772934768773368014908048319455582421828368909088093894782140;
    uint256 constant IC1y = 13648924575207057205828710936266196019338592610844972444359309060671194627724;

    uint256 constant IC2x = 11352535123750801305378113993219566070406269684078143318856379222802673541843;
    uint256 constant IC2y = 18830741697678945484062999582235800213861136801717940082957712081356058685336;

    uint256 constant IC3x = 1886082373158188629059745068204687138385731826283829580637595623393683533301;
    uint256 constant IC3y = 13245243840790007116064531648616987974506460307985097864154872491984627213880;

    uint256 constant IC4x = 3123058228875020096170326548595852565410303666749526466495591608503433220136;
    uint256 constant IC4y = 11581539787430888731720739732738541881977540869623546259434875884134350723488;

    uint256 constant IC5x = 5990024776484269725805903790505801399590145996032904727590349007069446433599;
    uint256 constant IC5y = 3789589906542186479645071611009232744684412686806625892886938902833553844385;

    uint256 constant IC6x = 1804450120767801357658552377309384805899555191385441966887131003075353744613;
    uint256 constant IC6y = 16457807631132919440195991620317626427119900133914581492032858682353087659565;

    uint256 constant IC7x = 11128625414184959403032828462902556663305215197629715252588791515485273426347;
    uint256 constant IC7y = 14078057825435677277115259711358672599737588205620464203029016162868563017273;

    uint256 constant IC8x = 3593570510084729307363516196279617608990143872801271859474288490525795842146;
    uint256 constant IC8y = 12000812784901719597310552308622099856079920617104811195654816646122719385534;

    uint256 constant IC9x = 17894978272272248217114465246028446294755715623570483004630158168446873035722;
    uint256 constant IC9y = 9497182833100040499334537546436605111158450588089155868657678403246228386492;

    uint256 constant IC10x = 2201168571758591872613066673392084311750780515393021744682593374549314581649;
    uint256 constant IC10y = 12260015204188265972237799996566279780373142070586293142366518631139088720377;

    uint256 constant IC11x = 6884516010180175565015729693491315393058661894640899937264300192068519860755;
    uint256 constant IC11y = 19893121072683080755403712509298606771535324885100373608868458944974723623994;

    uint256 constant IC12x = 4385672833045916098408168199961310671442709366492395626554558132557233201782;
    uint256 constant IC12y = 9571539912035074553925056956190290324534884914941432089751219380549376460522;

    uint256 constant IC13x = 9378556648752242218194734132675960483301983143287795769806238068880734968488;
    uint256 constant IC13y = 17570545094183593323131263108837039848933551384776188444224372647254454410332;

    uint256 constant IC14x = 20523272377230245586072050549779969882568320359824809390745825467061200286149;
    uint256 constant IC14y = 13893566106742523614534951509746881470284732973710727699260341448534955124598;

    uint256 constant IC15x = 2962707707507333521510303987898416641230432975955316358353499043622620019013;
    uint256 constant IC15y = 21070940068455551038205161298362591847991220887950977528179361001368608401499;

    uint256 constant IC16x = 8618253523714373055157263719885550336081269004896815522656609363965930772587;
    uint256 constant IC16y = 11094953650436261930807953459037725369273506572016070681444256073201847301625;

    uint256 constant IC17x = 10477069269574308792027591221646762376280632591446853826229640483403476610331;
    uint256 constant IC17y = 81320506023244774214118576512431249060973919652131058235371300050638471923;

    uint256 constant IC18x = 21698819389438636005950273274375624965505851323328610463491129873013253538944;
    uint256 constant IC18y = 2689599776875919058980481585698572437648874200754949224302098877226044404143;

    uint256 constant IC19x = 20942596650132352302835668870482584062707336543692678335740828247826799018198;
    uint256 constant IC19y = 6804640475984173269818996216440822417438809235678673260798349580428826003226;

    uint256 constant IC20x = 20479003413090217214404232350833497549250504057857160076231406597567857350814;
    uint256 constant IC20y = 15425166297776496285555989349656645134941490970111947538838107219819657883702;

    uint256 constant IC21x = 9653017296137887999807409724933499702605520806072333387152961175613275803457;
    uint256 constant IC21y = 6245067118677637506579622995905747392374480596379632820072449454896097478871;

    uint256 constant IC22x = 13589472542773933686575880891734590426327587821344441869050530704545049504910;
    uint256 constant IC22y = 3466996902464762263956157580504962843608389330226137863747243992172357895958;

    uint256 constant IC23x = 10178892551820160112115483798637772977943009047194043387482880234435264945636;
    uint256 constant IC23y = 19770375883684263161299329522460088899929492266016081741451020405265088743166;

    uint256 constant IC24x = 3621667108285591128094489659807800719296509879500865530846627787423444496315;
    uint256 constant IC24y = 20882152035419050317482883202689171402437112784141360963272347225160054107684;

    uint256 constant IC25x = 20687725993617328532380645185147480784815852441010189701944012200561160078814;
    uint256 constant IC25y = 9299093878200913944245516909451905062639707167811948945986542006155502099732;

    uint256 constant IC26x = 6731185520157926189820615544973535987557498273835653845236942555650418765887;
    uint256 constant IC26y = 15998282320144519008219322795718391788997323178213457240898592808837315068122;

    uint256 constant IC27x = 18805920385226285703240756202121185202772615145030920925274175895141604957342;
    uint256 constant IC27y = 2886958960884680744087928056352495364630535673856928113371725128925112604507;

    uint256 constant IC28x = 17765380779154759621748446154623028409694077330835000681269825700151156479619;
    uint256 constant IC28y = 10954553836932575547396734427554793825297428796095623342891635344250636221125;

    uint256 constant IC29x = 18907407196060311846807273685977428575573821025128992713865560136676809332243;
    uint256 constant IC29y = 12632864799941181181093883964846366158256711974952593641874733574524134101014;

    uint256 constant IC30x = 20203417079409580802460785524499464543743325791005733621733440005052388098278;
    uint256 constant IC30y = 12083403216978021089811224356763617398570468093699946538253570044730991419020;

    uint256 constant IC31x = 647177058227927215980787857473999618718058372856439371715943895546233058285;
    uint256 constant IC31y = 11651450653940347413433815765507084661480460836246074144384650786320590583490;

    uint256 constant IC32x = 9981265091545383506645389639016566412109283775809109149875205697981087699380;
    uint256 constant IC32y = 11537723551125121931312335247743376977844918739526961019249916546240078801960;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[32] calldata _pubSignals
    ) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, r)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x

                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))

                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))

                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))

                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))

                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))

                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))

                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))

                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))

                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))

                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))

                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))

                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))

                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))

                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))

                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))

                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))

                g1_mulAccC(_pVk, IC17x, IC17y, calldataload(add(pubSignals, 512)))

                g1_mulAccC(_pVk, IC18x, IC18y, calldataload(add(pubSignals, 544)))

                g1_mulAccC(_pVk, IC19x, IC19y, calldataload(add(pubSignals, 576)))

                g1_mulAccC(_pVk, IC20x, IC20y, calldataload(add(pubSignals, 608)))

                g1_mulAccC(_pVk, IC21x, IC21y, calldataload(add(pubSignals, 640)))

                g1_mulAccC(_pVk, IC22x, IC22y, calldataload(add(pubSignals, 672)))

                g1_mulAccC(_pVk, IC23x, IC23y, calldataload(add(pubSignals, 704)))

                g1_mulAccC(_pVk, IC24x, IC24y, calldataload(add(pubSignals, 736)))

                g1_mulAccC(_pVk, IC25x, IC25y, calldataload(add(pubSignals, 768)))

                g1_mulAccC(_pVk, IC26x, IC26y, calldataload(add(pubSignals, 800)))

                g1_mulAccC(_pVk, IC27x, IC27y, calldataload(add(pubSignals, 832)))

                g1_mulAccC(_pVk, IC28x, IC28y, calldataload(add(pubSignals, 864)))

                g1_mulAccC(_pVk, IC29x, IC29y, calldataload(add(pubSignals, 896)))

                g1_mulAccC(_pVk, IC30x, IC30y, calldataload(add(pubSignals, 928)))

                g1_mulAccC(_pVk, IC31x, IC31y, calldataload(add(pubSignals, 960)))

                g1_mulAccC(_pVk, IC32x, IC32y, calldataload(add(pubSignals, 992)))

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))

                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)

                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F

            checkField(calldataload(add(_pubSignals, 0)))

            checkField(calldataload(add(_pubSignals, 32)))

            checkField(calldataload(add(_pubSignals, 64)))

            checkField(calldataload(add(_pubSignals, 96)))

            checkField(calldataload(add(_pubSignals, 128)))

            checkField(calldataload(add(_pubSignals, 160)))

            checkField(calldataload(add(_pubSignals, 192)))

            checkField(calldataload(add(_pubSignals, 224)))

            checkField(calldataload(add(_pubSignals, 256)))

            checkField(calldataload(add(_pubSignals, 288)))

            checkField(calldataload(add(_pubSignals, 320)))

            checkField(calldataload(add(_pubSignals, 352)))

            checkField(calldataload(add(_pubSignals, 384)))

            checkField(calldataload(add(_pubSignals, 416)))

            checkField(calldataload(add(_pubSignals, 448)))

            checkField(calldataload(add(_pubSignals, 480)))

            checkField(calldataload(add(_pubSignals, 512)))

            checkField(calldataload(add(_pubSignals, 544)))

            checkField(calldataload(add(_pubSignals, 576)))

            checkField(calldataload(add(_pubSignals, 608)))

            checkField(calldataload(add(_pubSignals, 640)))

            checkField(calldataload(add(_pubSignals, 672)))

            checkField(calldataload(add(_pubSignals, 704)))

            checkField(calldataload(add(_pubSignals, 736)))

            checkField(calldataload(add(_pubSignals, 768)))

            checkField(calldataload(add(_pubSignals, 800)))

            checkField(calldataload(add(_pubSignals, 832)))

            checkField(calldataload(add(_pubSignals, 864)))

            checkField(calldataload(add(_pubSignals, 896)))

            checkField(calldataload(add(_pubSignals, 928)))

            checkField(calldataload(add(_pubSignals, 960)))

            checkField(calldataload(add(_pubSignals, 992)))

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
