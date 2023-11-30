use std::fs::File;
use std::io::prelude::*;

pub fn get_data() -> String {
    let mut data = String::new();
    let mut file = File::open("data.txt").expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read file");
    data
}

pub fn hard_coded_data() -> String {
    String::from("11223
    6323
    10725
    10761
    3587
    
    1274
    1041
    5566
    1759
    1372
    1619
    2228
    1283
    1981
    1885
    5894
    1321
    6081
    4407
    2992
    
    7184
    2310
    7975
    2752
    7942
    7616
    3622
    1320
    1231
    6191
    
    3069
    10691
    4789
    5329
    6726
    9550
    6188
    3117
    
    17492
    3489
    18055
    
    12369
    21123
    
    2121
    5065
    5377
    1768
    6060
    1256
    1496
    6899
    3569
    3766
    6873
    5490
    2961
    
    6407
    3770
    2823
    5998
    7783
    4194
    1866
    5504
    6550
    7857
    2884
    
    5385
    4621
    2434
    6123
    2159
    6191
    3552
    6871
    
    6531
    4287
    7402
    5634
    3960
    
    4955
    6958
    7652
    7416
    7540
    7990
    1696
    2568
    1040
    3087
    5608
    
    28948
    24385
    
    27643
    31191
    
    8674
    10486
    1765
    1787
    4234
    4187
    7126
    
    6370
    1098
    2071
    4231
    3223
    1456
    3853
    5740
    6327
    2723
    4716
    
    6783
    5772
    4352
    2984
    3483
    2359
    7168
    1581
    6182
    1637
    2601
    4015
    
    12691
    14785
    18295
    
    1035
    5658
    8643
    2693
    6046
    1531
    6174
    8290
    5239
    3430
    
    2510
    1090
    6971
    5314
    6949
    8885
    3334
    9329
    2738
    
    23893
    19568
    19775
    
    4257
    5830
    3972
    4513
    3988
    7674
    1535
    5402
    6257
    1785
    
    12580
    
    3869
    4718
    7144
    7107
    4168
    5713
    6920
    5802
    3226
    1389
    5876
    
    3405
    1630
    3297
    1130
    4787
    2319
    5745
    5744
    2852
    5594
    1641
    2012
    3828
    3717
    2181
    
    6200
    4566
    5973
    4301
    1608
    7023
    6922
    6247
    4402
    6943
    4980
    
    6063
    4381
    3878
    2147
    1437
    2822
    5830
    2453
    3898
    6106
    4618
    1905
    1608
    5115
    1656
    
    7057
    6579
    5295
    6106
    5517
    5560
    2264
    5144
    5623
    1650
    6594
    5828
    
    8444
    4858
    8810
    6516
    5064
    1198
    6087
    8280
    8275
    5471
    
    9667
    26851
    
    5191
    5421
    2865
    1495
    5491
    3406
    1779
    2454
    6648
    6237
    1868
    1313
    
    6885
    8414
    6097
    
    9459
    9028
    2432
    4442
    1721
    8546
    5060
    
    8042
    13365
    7153
    10483
    7147
    
    11597
    1891
    10089
    9866
    11989
    1774
    1386
    
    7690
    5023
    5273
    5257
    1790
    2008
    1100
    4395
    3691
    1270
    
    34927
    32922
    
    22524
    19581
    7007
    
    6204
    5778
    2172
    3231
    5452
    4320
    5110
    1397
    4063
    2370
    6075
    3506
    3764
    2327
    
    1764
    1282
    1837
    3490
    4161
    5852
    5595
    1428
    4863
    1894
    2983
    2336
    4587
    1656
    5178
    
    3987
    6437
    4976
    3214
    2354
    7292
    1167
    4874
    6006
    
    6911
    3746
    6773
    7196
    7615
    1359
    3757
    7858
    5074
    6991
    5915
    
    4952
    6034
    4897
    3990
    5389
    1886
    3544
    7289
    2881
    2402
    6238
    6533
    
    4548
    5600
    5383
    3044
    1113
    6264
    2151
    6527
    7206
    2516
    6959
    7111
    
    5811
    4157
    5365
    3234
    5614
    5556
    3486
    2452
    4304
    4990
    2269
    2944
    2060
    4504
    
    67337
    
    10762
    7277
    4855
    8338
    2555
    4499
    9727
    
    7093
    6258
    
    2117
    6720
    2717
    1336
    2124
    7329
    1182
    4312
    7579
    3997
    
    11769
    4034
    4675
    3409
    5871
    1752
    5233
    
    4359
    7926
    8543
    4196
    3184
    9262
    7649
    
    11423
    1963
    11334
    11792
    
    5951
    5005
    2263
    6667
    1586
    6799
    7104
    5810
    
    1474
    1433
    4979
    5569
    5195
    5467
    5503
    6305
    4527
    5721
    4151
    4965
    3459
    
    1572
    4083
    3559
    3119
    4290
    5019
    1274
    1243
    5732
    3525
    2310
    4945
    1128
    2249
    5974
    
    2324
    6525
    6790
    5283
    4011
    1907
    2385
    6918
    5300
    1177
    5483
    6271
    
    2380
    8204
    6100
    3828
    4024
    5895
    1613
    4301
    4242
    4265
    
    3519
    1809
    8065
    3966
    7829
    1476
    5391
    2276
    5886
    5447
    5325
    
    4424
    3641
    1053
    1843
    2247
    2337
    3572
    1720
    4668
    
    7399
    7282
    3897
    6376
    3540
    5313
    6058
    5209
    5392
    5429
    
    4724
    6793
    7620
    4771
    8430
    2765
    10691
    
    9003
    5417
    7064
    3317
    2271
    3973
    2491
    5203
    
    1978
    5937
    5166
    1587
    5342
    1353
    2506
    3213
    2041
    4374
    3097
    1710
    
    2998
    7694
    2359
    6486
    3319
    7771
    5340
    8062
    6645
    
    6121
    5006
    3579
    12148
    3412
    8373
    10656
    
    2851
    13441
    
    2361
    7577
    8360
    2128
    4313
    6552
    5422
    8627
    
    3435
    11683
    4085
    11581
    3017
    4731
    8645
    
    10115
    9761
    7367
    5257
    10263
    5022
    9674
    6552
    
    4997
    2470
    2855
    1860
    2547
    3060
    1036
    3288
    6338
    5423
    4701
    6740
    2226
    
    2182
    2468
    2724
    4116
    2740
    1677
    1642
    2717
    1604
    2874
    4231
    5637
    5907
    1780
    5585
    
    1425
    5503
    8059
    2459
    2334
    1655
    10561
    
    7278
    5393
    6827
    1776
    6841
    6404
    7090
    7051
    1764
    2107
    5593
    3639
    
    17501
    16452
    11047
    7696
    
    11332
    9288
    13543
    2406
    7699
    5661
    
    9845
    14017
    12535
    7383
    
    4341
    1266
    3942
    2669
    1219
    1987
    5423
    2973
    5656
    4981
    3084
    4455
    1780
    1772
    2947
    
    6183
    2669
    6038
    8553
    12075
    
    3041
    6281
    2661
    1855
    6460
    6508
    6582
    5775
    3895
    1141
    1096
    3572
    
    1458
    2196
    4078
    1940
    2785
    2560
    6964
    5713
    7913
    6951
    2396
    
    1820
    3990
    16654
    15798
    
    1445
    1269
    2821
    5078
    3402
    4371
    6619
    3479
    5353
    1921
    5645
    3800
    
    11876
    15581
    11141
    17267
    
    9397
    8627
    3948
    4177
    6374
    4943
    7444
    4766
    5053
    
    6847
    8418
    2322
    2549
    1334
    5677
    1151
    8407
    
    69814
    
    1529
    5847
    8312
    7219
    4067
    7646
    5212
    5183
    9533
    
    17696
    2661
    7171
    
    8696
    10434
    5330
    6274
    5824
    2386
    10535
    8402
    
    3339
    9444
    3712
    4540
    4296
    4385
    2453
    10416
    
    4225
    7128
    7768
    7085
    1382
    4348
    6967
    1719
    3131
    4060
    3516
    
    3329
    3612
    1093
    4118
    2946
    2250
    1200
    2193
    4142
    3120
    2394
    1217
    1214
    2310
    3690
    
    9634
    10560
    3796
    7011
    10745
    10677
    
    4024
    4466
    6953
    1754
    2261
    6560
    6690
    2671
    6082
    5522
    4024
    
    16675
    17642
    1382
    12630
    
    4454
    4023
    6985
    6880
    5862
    3351
    2861
    5649
    6833
    4668
    2089
    3832
    
    3744
    2151
    3951
    3859
    1900
    2921
    1267
    2132
    4015
    3160
    4747
    4780
    4398
    4160
    5221
    
    18994
    14365
    11726
    
    8313
    16460
    18955
    7491
    
    7558
    1673
    1267
    4825
    4651
    5819
    9798
    
    50633
    
    4542
    1002
    4677
    4874
    5472
    3518
    3642
    3083
    4735
    6769
    5389
    2869
    4223
    
    4275
    4306
    4645
    2360
    4603
    7805
    7964
    8022
    7382
    6504
    
    2452
    9071
    6449
    11056
    7775
    
    8750
    8343
    3753
    11284
    2024
    1088
    
    2376
    8200
    1858
    11313
    10710
    5992
    
    32813
    26299
    
    15734
    8393
    7230
    20011
    
    19297
    21723
    14837
    
    2126
    3511
    6139
    3427
    6135
    5685
    1514
    4338
    4446
    3334
    4417
    1343
    
    5737
    1004
    1168
    4504
    2467
    3431
    3800
    1931
    5287
    4057
    6507
    4662
    1845
    
    6896
    5740
    6075
    5798
    6055
    4309
    5679
    7279
    1311
    4647
    
    3594
    7095
    7139
    6593
    9371
    6162
    1320
    7202
    6216
    
    3592
    1250
    2848
    6524
    1240
    6845
    5263
    1176
    1165
    3641
    5653
    3051
    6168
    
    5535
    4847
    13599
    5746
    5288
    7316
    
    13934
    13998
    25642
    
    4070
    1691
    6012
    5137
    3924
    2272
    4886
    4272
    3467
    5077
    3114
    3794
    5220
    1129
    2113
    
    8763
    11869
    7453
    2318
    10101
    5238
    4663
    
    17968
    21896
    25750
    
    2567
    6997
    8972
    4451
    4917
    4229
    5235
    8327
    2785
    
    6031
    17803
    19811
    
    5995
    3180
    2417
    5245
    2481
    2578
    3879
    4348
    1183
    2027
    1809
    1278
    3428
    2253
    5083
    
    5958
    8942
    6761
    4251
    1630
    2788
    7080
    6954
    4471
    
    4874
    2390
    4410
    1109
    5096
    6956
    1676
    4848
    3521
    5799
    5997
    3748
    5709
    
    3624
    7400
    4652
    3504
    2104
    6861
    4377
    1647
    4293
    7214
    7099
    5289
    
    10481
    1917
    
    3507
    5672
    2015
    2831
    6385
    4068
    5241
    7465
    4759
    5053
    7052
    
    6280
    9794
    4646
    6980
    3720
    
    8585
    9331
    3157
    9319
    7640
    4446
    5594
    1358
    
    6644
    9767
    6051
    6421
    8044
    5769
    1292
    6633
    
    1230
    1380
    4214
    4932
    5426
    3025
    4739
    2440
    4998
    2899
    3845
    2650
    2980
    6266
    
    6378
    1563
    3315
    4910
    1256
    9085
    5390
    10324
    
    8771
    4691
    1164
    1066
    9470
    4868
    9659
    7827
    
    1105
    6108
    4769
    3728
    1575
    4603
    6185
    6376
    5012
    5673
    1280
    5428
    6304
    
    2239
    1560
    5668
    4430
    3985
    4435
    5807
    4135
    5911
    4154
    2544
    2498
    5446
    4814
    
    14668
    11210
    13344
    12364
    9961
    
    11641
    1360
    7500
    4268
    9701
    4171
    2548
    
    1947
    3352
    5709
    4353
    4671
    3119
    2870
    2138
    4595
    2051
    1807
    3416
    3492
    1334
    4695
    
    7970
    8487
    1319
    1471
    7599
    8547
    6998
    1005
    1741
    
    4692
    1989
    2772
    4105
    2154
    1885
    4124
    6762
    5784
    7407
    2898
    
    3169
    2220
    5328
    4295
    8593
    3355
    4605
    6313
    1871
    
    10304
    9352
    7552
    9610
    1175
    10454
    3342
    2821
    
    16100
    9638
    8215
    10806
    3023
    
    29092
    1901
    
    8203
    13092
    10935
    13568
    11032
    2819
    
    4185
    6055
    1638
    4727
    2223
    1707
    2701
    2955
    5282
    1704
    1728
    3464
    4085
    4768
    4239
    
    5205
    7478
    1929
    2900
    5997
    
    34467
    23210
    
    5963
    1701
    6455
    9686
    8353
    5662
    3429
    6921
    9281
    
    4888
    4749
    6983
    4976
    2664
    9637
    5002
    2791
    
    10256
    25711
    22494
    
    3115
    2602
    3274
    4413
    4512
    1994
    3181
    3316
    1755
    2764
    5607
    1058
    4609
    1329
    6055
    
    6595
    4222
    3407
    10124
    1446
    4008
    7234
    
    3194
    4401
    4598
    3461
    2599
    6216
    4501
    1280
    4413
    6073
    1774
    5722
    3007
    3088
    
    10411
    25669
    8871
    
    2613
    2848
    2537
    1277
    5831
    6144
    5639
    6685
    6095
    3826
    1796
    6333
    6555
    
    1310
    6643
    9862
    2064
    1697
    4078
    3647
    6155
    
    4361
    2370
    2349
    2015
    4278
    4873
    5165
    4172
    3229
    6081
    5611
    4981
    5538
    4877
    3416
    
    4484
    6999
    7901
    7691
    2688
    5054
    5847
    10641
    
    49205
    
    4445
    3962
    4702
    5865
    4458
    4673
    4979
    2816
    4194
    5485
    1370
    1414
    1938
    6069
    1713
    
    1272
    5551
    6047
    1084
    1476
    6923
    3815
    2695
    3753
    4342
    4076
    7109
    
    1166
    10620
    2255
    15911
    
    3776
    31838
    
    5462
    7846
    10506
    10139
    
    5095
    7569
    5193
    2026
    7823
    7835
    8264
    3499
    9567
    
    8657
    6625
    5734
    6420
    3489
    8213
    2147
    7071
    3228
    6418
    
    10360
    1069
    14776
    6090
    
    5317
    3597
    5311
    2456
    1009
    3954
    5518
    3283
    3619
    4970
    5877
    4643
    5428
    2472
    1800
    
    9343
    8166
    3967
    4650
    6978
    1809
    9866
    1354
    
    5664
    5951
    
    18203
    7380
    2701
    16860
    
    2995
    2739
    5498
    1567
    8326
    2770
    4838
    
    10006
    1510
    3844
    1706
    8009
    11790
    7550
    
    1185
    2494
    1299
    4168
    6869
    4108
    4941
    5636
    2668
    4842
    3448
    3087
    3871
    
    4568
    21522
    11228
    
    1210
    5132
    6540
    8377
    5968
    5358
    4232
    4710
    
    6603
    23736
    
    4328
    2190
    4021
    4316
    4760
    3010
    5604
    4746
    5843
    5327
    2718
    4956
    3167
    3359
    
    8760
    6848
    7827
    7235
    2491
    6362
    7344
    4841
    4454
    8604
    
    8394
    5794
    13797
    15870
    4818
    
    8412
    14825
    16385
    11468
    
    29542
    
    6924
    5318
    6833
    3875
    6931
    3650
    2849
    2068
    7369
    1375
    2375
    4576
    
    2269
    5568
    4133
    3853
    1996
    5885
    1074
    3356
    5187
    6079
    1522
    5220
    1704
    2942
    1641
    
    14866
    9313
    23116
    
    13359
    13222
    15441
    14439
    16050
    
    8511
    10467
    8505
    2880
    4654
    6594
    3942
    1306
    
    6483
    7197
    4990
    6774
    
    8594
    7425
    5701
    9526
    2538
    9519
    8284
    8558
    9647
    
    1181
    7034
    13744
    6912
    9743
    8735
    
    5076
    7046
    5565
    2810
    1461
    4916
    3777
    5174
    1842
    6263
    4151
    6307
    
    6262
    11239
    
    2639
    3859
    6504
    4947
    6810
    6805
    3288
    3042
    5873
    6339
    2931
    4332
    1707
    
    1834
    2381
    5243
    5769
    3444
    2755
    4929
    1520
    3791
    2443
    1521
    6059
    3943
    5845
    
    19725
    18762
    2318
    11981
    
    7867
    10660
    2834
    13236
    1524
    7225
    
    1377
    2138
    2955
    1407
    3971
    1872
    1293
    6090
    1203
    6798
    2729
    3360
    
    12399
    5994
    7732
    12299
    12160
    
    10151
    19182
    18060
    19397
    
    34075
    25670
    
    5145
    6043
    5241
    2001
    6063
    1342
    4769
    6629
    7465
    3500
    4873
    
    4825
    5192
    1670
    5946
    8053
    2622
    1436
    5977
    4523
    4415
    5831
    
    7035
    3322
    7624
    1840
    2078
    6420
    6273
    4362
    4874
    7712
    5714
    
    3644
    4526
    2940
    7904
    7620
    6260
    4794
    3705
    3585
    3033
    
    5391
    6000
    3398
    3922
    4655
    6747
    6505
    3402
    2082
    1856
    6626
    
    6085
    5246
    3072
    8802
    3666
    6219
    7612
    4531
    
    5420
    2366
    3861
    5686
    2737
    6276
    2292
    6004
    6195
    1605
    1639
    7004
    
    4932
    2314
    2297
    1611
    4624
    7495
    3485
    6580
    1751
    5170
    6100
    
    9273
    11201
    6597
    3958
    11219
    8899
    9413
    
    28386
    26100
    
    3170
    2290
    6393
    2973
    5677
    4855
    1937
    4071
    2830
    2344
    4708
    3775
    2780
    5562
    
    11208
    4958
    7770
    10314
    8593
    11973
    6000
    
    19570
    24273
    20898
    
    1972
    12111
    8510
    1145
    
    12217
    13432
    
    7383
    1407
    5216
    8061
    10559
    8810
    1385
    
    2300
    3845
    4200
    3033
    2758
    2464
    1543
    1216
    4980
    4409
    3881
    5162
    1743
    2350
    4386
    
    5339
    2610
    10101
    3159
    15809
    
    6064
    3176
    6116
    1092
    4494
    1245
    4343
    2704
    5405
    5426
    5126
    4722
    1609
    4367
    2280
    
    16523
    23950
    11677
    
    9674
    1499
    11422
    13298
    9949
    10616
    
    3895
    9314
    12341
    8416
    
    1949
    9055
    3289
    5458
    9902
    7137
    6167
    5111
    
    2141
    2700
    2221
    7184
    6103
    4030
    3695
    4116
    3358
    6650
    5498
    1608
    
    3900
    5189
    3383
    5800
    5109
    2383
    1001
    6074
    1110
    5187
    5430
    3448
    1899
    5865
    1098
    
    8948
    2222
    8881
    1948
    6993
    4242
    4588
    7172
    
    6514
    3993
    9681
    6992
    6150
    10477
    5089
    8951
    
    1714
    7959
    12561
    13204
    13321
    3506
    
    4979
    3299
    1108
    5065
    
    5536
    8474
    8551
    5400
    1405
    10623
    5028
    9292
    
    1199
    4551
    4953
    2814
    4224
    6361
    3457
    3640
    3139
    4825
    2001
    1432
    4116
    3688
    
    13664
    10207
    6028
    3098
    4410
    4175
    
    5623
    3796
    2402
    4038
    2083
    2761
    1806
    3806
    1704
    1905
    1550
    6133
    4524
    
    3052
    3739
    3842
    7695
    8135
    3738
    4717
    3232
    8761
    
    4498
    4685
    7377
    2806
    5031
    3386
    1609
    7810
    3717
    2535
    6173
    
    15301
    1343
    
    6134
    
    3331
    8838
    18585
    2670
    
    10770
    4959
    8095
    1497
    1627
    4809
    5857
    10329
    
    8734
    3895
    4301
    5958
    1740
    10403
    8830
    
    3531
    13028
    12682
    
    19031
    
    14044
    
    2867
    1941
    5744
    9099
    3583
    1719
    3917
    2737
    4593
    
    4474
    2684
    9230
    7981
    3032
    4811
    9558
    8513
    
    23797
    27967
    
    9143
    8743
    9120
    4760
    1985
    7971
    7476
    10415
    
    24714
    13638
    
    13753
    13010
    12301
    15112
    13584
    
    1467
    2681
    4482
    3424
    7161
    7828
    6262
    5660
    6950
    5807
    5109
    
    3023
    5008
    5278
    1131
    1563
    6917
    1553
    2406
    3173
    3255
    2772
    1427
    1930")
}