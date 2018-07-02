use combine::{
    count,
    parser::{
        item::satisfy,
        char::{
            char as c_char,
            hex_digit,
        }
    },
    Parser,
    error::ParseError,
    Stream
};

pub fn ll<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        (raw_char >= 97 && raw_char <= 122)
        || raw_char == 181 || (raw_char >= 223 && raw_char <= 246)
        || (raw_char >= 248 && raw_char <= 255)
        || raw_char == 257 || raw_char == 259 || raw_char == 261 || raw_char == 263
        || raw_char == 265 || raw_char == 267 || raw_char == 269 || raw_char == 271
        || raw_char == 273
        || raw_char == 275 || raw_char == 277 || raw_char == 279 || raw_char == 281 || raw_char == 283
        || raw_char == 285 || raw_char == 287 || raw_char == 289 || raw_char == 291 || raw_char == 293
        || raw_char == 295 || raw_char == 297 || raw_char == 299 || raw_char == 301 || raw_char == 303
        || raw_char == 305 || raw_char == 307 || raw_char == 309 || (raw_char >= 311 && raw_char <= 312)
        || raw_char == 314 || raw_char == 316 || raw_char == 318 || raw_char == 320 || raw_char == 322
        || raw_char == 324 || raw_char == 326 || (raw_char >= 328 && raw_char <= 329)
        || raw_char == 331
    || raw_char == 333
    || raw_char == 335
    || raw_char == 337
    || raw_char == 339
    || raw_char == 341
    || raw_char == 343
    || raw_char == 345
    || raw_char == 347
    || raw_char == 349
    || raw_char == 351
    || raw_char == 353
    || raw_char == 355
    || raw_char == 357
    || raw_char == 359
    || raw_char == 361
    || raw_char == 363
    || raw_char == 365
    || raw_char == 367
    || raw_char == 369
    || raw_char == 371
    || raw_char == 373
    || raw_char == 375
    || raw_char == 378
    || raw_char == 380
    || (raw_char >= 382 && raw_char <= 384)
    || raw_char == 387
    || raw_char == 389
    || raw_char == 392
    || (raw_char >= 396 && raw_char <= 397)
    || raw_char == 402
    || raw_char == 405
    || (raw_char >= 409 && raw_char <= 411)
    || raw_char == 414
    || raw_char == 417
    || raw_char == 419
    || raw_char == 421
    || raw_char == 424
    || (raw_char >= 426 && raw_char <= 427)
    || raw_char == 429
    || raw_char == 432
    || raw_char == 436
    || raw_char == 438
    || (raw_char >= 441 && raw_char <= 442)
    || (raw_char >= 445 && raw_char <= 447)
    || raw_char == 454
    || raw_char == 457
    || raw_char == 460
    || raw_char == 462
    || raw_char == 464
    || raw_char == 466
    || raw_char == 468
    || raw_char == 470
    || raw_char == 472
    || raw_char == 474
    || (raw_char >= 476 && raw_char <= 477)
    || raw_char == 479
    || raw_char == 481
    || raw_char == 483
    || raw_char == 485
    || raw_char == 487
    || raw_char == 489
    || raw_char == 491
    || raw_char == 493
    || (raw_char >= 495 && raw_char <= 496)
    || raw_char == 499
    || raw_char == 501
    || raw_char == 505
    || raw_char == 507
    || raw_char == 509
    || raw_char == 511
    || raw_char == 513
    || raw_char == 515
    || raw_char == 517
    || raw_char == 519
    || raw_char == 521
    || raw_char == 523
    || raw_char == 525
    || raw_char == 527
    || raw_char == 529
    || raw_char == 531
    || raw_char == 533
    || raw_char == 535
    || raw_char == 537
    || raw_char == 539
    || raw_char == 541
    || raw_char == 543
    || raw_char == 545
    || raw_char == 547
    || raw_char == 549
    || raw_char == 551
    || raw_char == 553
    || raw_char == 555
    || raw_char == 557
    || raw_char == 559
    || raw_char == 561
    || (raw_char >= 563 && raw_char <= 569)
    || raw_char == 572
    || (raw_char >= 575 && raw_char <= 576)
    || raw_char == 578
    || raw_char == 583
    || raw_char == 585
    || raw_char == 587
    || raw_char == 589
    || (raw_char >= 591 && raw_char <= 659)
    || (raw_char >= 661 && raw_char <= 687)
    || raw_char == 881
    || raw_char == 883
    || raw_char == 887
    || (raw_char >= 891 && raw_char <= 893)
    || raw_char == 912
    || (raw_char >= 940 && raw_char <= 974)
    || (raw_char >= 976 && raw_char <= 977)
    || (raw_char >= 981 && raw_char <= 983)
    || raw_char == 985
    || raw_char == 987
    || raw_char == 989
    || raw_char == 991
    || raw_char == 993
    || raw_char == 995
    || raw_char == 997
    || raw_char == 999
    || raw_char == 1001
    || raw_char == 1003
    || raw_char == 1005
    || (raw_char >= 1007 && raw_char <= 1011)
    || raw_char == 1013
    || raw_char == 1016
    || (raw_char >= 1019 && raw_char <= 1020)
    || (raw_char >= 1072 && raw_char <= 1119)
    || raw_char == 1121
    || raw_char == 1123
    || raw_char == 1125
    || raw_char == 1127
    || raw_char == 1129
    || raw_char == 1131
    || raw_char == 1133
    || raw_char == 1135
    || raw_char == 1137
    || raw_char == 1139
    || raw_char == 1141
    || raw_char == 1143
    || raw_char == 1145
    || raw_char == 1147
    || raw_char == 1149
    || raw_char == 1151
    || raw_char == 1153
    || raw_char == 1163
    || raw_char == 1165
    || raw_char == 1167
    || raw_char == 1169
    || raw_char == 1171
    || raw_char == 1173
    || raw_char == 1175
    || raw_char == 1177
    || raw_char == 1179
    || raw_char == 1181
    || raw_char == 1183
    || raw_char == 1185
    || raw_char == 1187
    || raw_char == 1189
    || raw_char == 1191
    || raw_char == 1193
    || raw_char == 1195
    || raw_char == 1197
    || raw_char == 1199
    || raw_char == 1201
    || raw_char == 1203
    || raw_char == 1205
    || raw_char == 1207
    || raw_char == 1209
    || raw_char == 1211
    || raw_char == 1213
    || raw_char == 1215
    || raw_char == 1218
    || raw_char == 1220
    || raw_char == 1222
    || raw_char == 1224
    || raw_char == 1226
    || raw_char == 1228
    || (raw_char >= 1230 && raw_char <= 1231)
    || raw_char == 1233
    || raw_char == 1235
    || raw_char == 1237
    || raw_char == 1239
    || raw_char == 1241
    || raw_char == 1243
    || raw_char == 1245
    || raw_char == 1247
    || raw_char == 1249
    || raw_char == 1251
    || raw_char == 1253
    || raw_char == 1255
    || raw_char == 1257
    || raw_char == 1259
    || raw_char == 1261
    || raw_char == 1263
    || raw_char == 1265
    || raw_char == 1267
    || raw_char == 1269
    || raw_char == 1271
    || raw_char == 1273
    || raw_char == 1275
    || raw_char == 1277
    || raw_char == 1279
    || raw_char == 1281
    || raw_char == 1283
    || raw_char == 1285
    || raw_char == 1287
    || raw_char == 1289
    || raw_char == 1291
    || raw_char == 1293
    || raw_char == 1295
    || raw_char == 1297
    || raw_char == 1299
    || raw_char == 1301
    || raw_char == 1303
    || raw_char == 1305
    || raw_char == 1307
    || raw_char == 1309
    || raw_char == 1311
    || raw_char == 1313
    || raw_char == 1315
    || raw_char == 1317
    || raw_char == 1319
    || raw_char == 1321
    || raw_char == 1323
    || raw_char == 1325
    || raw_char == 1327
    || (raw_char >= 1376 && raw_char <= 1416)
    || (raw_char >= 4304 && raw_char <= 4346)
    || (raw_char >= 4349 && raw_char <= 4351)
    || (raw_char >= 5112 && raw_char <= 5117)
    || (raw_char >= 7296 && raw_char <= 7304)
    || (raw_char >= 7424 && raw_char <= 7467)
    || (raw_char >= 7531 && raw_char <= 7543)
    || (raw_char >= 7545 && raw_char <= 7578)
    || raw_char == 7681
    || raw_char == 7683
    || raw_char == 7685
    || raw_char == 7687
    || raw_char == 7689
    || raw_char == 7691
    || raw_char == 7693
    || raw_char == 7695
    || raw_char == 7697
    || raw_char == 7699
    || raw_char == 7701
    || raw_char == 7703
    || raw_char == 7705
    || raw_char == 7707
    || raw_char == 7709
    || raw_char == 7711
    || raw_char == 7713
    || raw_char == 7715
    || raw_char == 7717
    || raw_char == 7719
    || raw_char == 7721
    || raw_char == 7723
    || raw_char == 7725
    || raw_char == 7727
    || raw_char == 7729
    || raw_char == 7731
    || raw_char == 7733
    || raw_char == 7735
    || raw_char == 7737
    || raw_char == 7739
    || raw_char == 7741
    || raw_char == 7743
    || raw_char == 7745
    || raw_char == 7747
    || raw_char == 7749
    || raw_char == 7751
    || raw_char == 7753
    || raw_char == 7755
    || raw_char == 7757
    || raw_char == 7759
    || raw_char == 7761
    || raw_char == 7763
    || raw_char == 7765
    || raw_char == 7767
    || raw_char == 7769
    || raw_char == 7771
    || raw_char == 7773
    || raw_char == 7775
    || raw_char == 7777
    || raw_char == 7779
    || raw_char == 7781
    || raw_char == 7783
    || raw_char == 7785
    || raw_char == 7787
    || raw_char == 7789
    || raw_char == 7791
    || raw_char == 7793
    || raw_char == 7795
    || raw_char == 7797
    || raw_char == 7799
    || raw_char == 7801
    || raw_char == 7803
    || raw_char == 7805
    || raw_char == 7807
    || raw_char == 7809
    || raw_char == 7811
    || raw_char == 7813
    || raw_char == 7815
    || raw_char == 7817
    || raw_char == 7819
    || raw_char == 7821
    || raw_char == 7823
    || raw_char == 7825
    || raw_char == 7827
    || (raw_char >= 7829 && raw_char <= 7837)
    || raw_char == 7839
    || raw_char == 7841
    || raw_char == 7843
    || raw_char == 7845
    || raw_char == 7847
    || raw_char == 7849
    || raw_char == 7851
    || raw_char == 7853
    || raw_char == 7855
    || raw_char == 7857
    || raw_char == 7859
    || raw_char == 7861
    || raw_char == 7863
    || raw_char == 7865
    || raw_char == 7867
    || raw_char == 7869
    || raw_char == 7871
    || raw_char == 7873
    || raw_char == 7875
    || raw_char == 7877
    || raw_char == 7879
    || raw_char == 7881
    || raw_char == 7883
    || raw_char == 7885
    || raw_char == 7887
    || raw_char == 7889
    || raw_char == 7891
    || raw_char == 7893
    || raw_char == 7895
    || raw_char == 7897
    || raw_char == 7899
    || raw_char == 7901
    || raw_char == 7903
    || raw_char == 7905
    || raw_char == 7907
    || raw_char == 7909
    || raw_char == 7911
    || raw_char == 7913
    || raw_char == 7915
    || raw_char == 7917
    || raw_char == 7919
    || raw_char == 7921
    || raw_char == 7923
    || raw_char == 7925
    || raw_char == 7927
    || raw_char == 7929
    || raw_char == 7931
    || raw_char == 7933
    || (raw_char >= 7935 && raw_char <= 7943)
    || (raw_char >= 7952 && raw_char <= 7957)
    || (raw_char >= 7968 && raw_char <= 7975)
    || (raw_char >= 7984 && raw_char <= 7991)
    || (raw_char >= 8000 && raw_char <= 8005)
    || (raw_char >= 8016 && raw_char <= 8023)
    || (raw_char >= 8032 && raw_char <= 8039)
    || (raw_char >= 8048 && raw_char <= 8061)
    || (raw_char >= 8064 && raw_char <= 8071)
    || (raw_char >= 8080 && raw_char <= 8087)
    || (raw_char >= 8096 && raw_char <= 8103)
    || (raw_char >= 8112 && raw_char <= 8116)
    || (raw_char >= 8118 && raw_char <= 8119)
    || raw_char == 8126
    || (raw_char >= 8130 && raw_char <= 8132)
    || (raw_char >= 8134 && raw_char <= 8135)
    || (raw_char >= 8144 && raw_char <= 8147)
    || (raw_char >= 8150 && raw_char <= 8151)
    || (raw_char >= 8160 && raw_char <= 8167)
    || (raw_char >= 8178 && raw_char <= 8180)
    || (raw_char >= 8182 && raw_char <= 8183)
    || raw_char == 8458
    || (raw_char >= 8462 && raw_char <= 8463)
    || raw_char == 8467
    || raw_char == 8495
    || raw_char == 8500
    || raw_char == 8505
    || (raw_char >= 8508 && raw_char <= 8509)
    || (raw_char >= 8518 && raw_char <= 8521)
    || raw_char == 8526
    || raw_char == 8580
    || (raw_char >= 11312 && raw_char <= 11358)
    || raw_char == 11361
    || (raw_char >= 11365 && raw_char <= 11366)
    || raw_char == 11368
    || raw_char == 11370
    || raw_char == 11372
    || raw_char == 11377
    || (raw_char >= 11379 && raw_char <= 11380)
    || (raw_char >= 11382 && raw_char <= 11387)
    || raw_char == 11393
    || raw_char == 11395
    || raw_char == 11397
    || raw_char == 11399
    || raw_char == 11401
    || raw_char == 11403
    || raw_char == 11405
    || raw_char == 11407
    || raw_char == 11409
    || raw_char == 11411
    || raw_char == 11413
    || raw_char == 11415
    || raw_char == 11417
    || raw_char == 11419
    || raw_char == 11421
    || raw_char == 11423
    || raw_char == 11425
    || raw_char == 11427
    || raw_char == 11429
    || raw_char == 11431
    || raw_char == 11433
    || raw_char == 11435
    || raw_char == 11437
    || raw_char == 11439
    || raw_char == 11441
    || raw_char == 11443
    || raw_char == 11445
    || raw_char == 11447
    || raw_char == 11449
    || raw_char == 11451
    || raw_char == 11453
    || raw_char == 11455
    || raw_char == 11457
    || raw_char == 11459
    || raw_char == 11461
    || raw_char == 11463
    || raw_char == 11465
    || raw_char == 11467
    || raw_char == 11469
    || raw_char == 11471
    || raw_char == 11473
    || raw_char == 11475
    || raw_char == 11477
    || raw_char == 11479
    || raw_char == 11481
    || raw_char == 11483
    || raw_char == 11485
    || raw_char == 11487
    || raw_char == 11489
    || (raw_char >= 11491 && raw_char <= 11492)
    || raw_char == 11500
    || raw_char == 11502
    || raw_char == 11507
    || (raw_char >= 11520 && raw_char <= 11557)
    || raw_char == 11559
    || raw_char == 11565
    || raw_char == 42561
    || raw_char == 42563
    || raw_char == 42565
    || raw_char == 42567
    || raw_char == 42569
    || raw_char == 42571
    || raw_char == 42573
    || raw_char == 42575
    || raw_char == 42577
    || raw_char == 42579
    || raw_char == 42581
    || raw_char == 42583
    || raw_char == 42585
    || raw_char == 42587
    || raw_char == 42589
    || raw_char == 42591
    || raw_char == 42593
    || raw_char == 42595
    || raw_char == 42597
    || raw_char == 42599
    || raw_char == 42601
    || raw_char == 42603
    || raw_char == 42605
    || raw_char == 42625
    || raw_char == 42627
    || raw_char == 42629
    || raw_char == 42631
    || raw_char == 42633
    || raw_char == 42635
    || raw_char == 42637
    || raw_char == 42639
    || raw_char == 42641
    || raw_char == 42643
    || raw_char == 42645
    || raw_char == 42647
    || raw_char == 42649
    || raw_char == 42651
    || raw_char == 42787
    || raw_char == 42789
    || raw_char == 42791
    || raw_char == 42793
    || raw_char == 42795
    || raw_char == 42797
    || (raw_char >= 42799 && raw_char <= 42801)
    || raw_char == 42803
    || raw_char == 42805
    || raw_char == 42807
    || raw_char == 42809
    || raw_char == 42811
    || raw_char == 42813
    || raw_char == 42815
    || raw_char == 42817
    || raw_char == 42819
    || raw_char == 42821
    || raw_char == 42823
    || raw_char == 42825
    || raw_char == 42827
    || raw_char == 42829
    || raw_char == 42831
    || raw_char == 42833
    || raw_char == 42835
    || raw_char == 42837
    || raw_char == 42839
    || raw_char == 42841
    || raw_char == 42843
    || raw_char == 42845
    || raw_char == 42847
    || raw_char == 42849
    || raw_char == 42851
    || raw_char == 42853
    || raw_char == 42855
    || raw_char == 42857
    || raw_char == 42859
    || raw_char == 42861
    || raw_char == 42863
    || (raw_char >= 42865 && raw_char <= 42872)
    || raw_char == 42874
    || raw_char == 42876
    || raw_char == 42879
    || raw_char == 42881
    || raw_char == 42883
    || raw_char == 42885
    || raw_char == 42887
    || raw_char == 42892
    || raw_char == 42894
    || raw_char == 42897
    || (raw_char >= 42899 && raw_char <= 42901)
    || raw_char == 42903
    || raw_char == 42905
    || raw_char == 42907
    || raw_char == 42909
    || raw_char == 42911
    || raw_char == 42913
    || raw_char == 42915
    || raw_char == 42917
    || raw_char == 42919
    || raw_char == 42921
    || raw_char == 42927
    || raw_char == 42933
    || raw_char == 42935
    || raw_char == 42937
    || raw_char == 43002
    || (raw_char >= 43824 && raw_char <= 43866)
    || (raw_char >= 43872 && raw_char <= 43877)
    || (raw_char >= 43888 && raw_char <= 43967)
    || (raw_char >= 64256 && raw_char <= 64262)
    || (raw_char >= 64275 && raw_char <= 64279)
    || (raw_char >= 65345 && raw_char <= 65370)
    || (raw_char >= 66600 && raw_char <= 66639)
    || (raw_char >= 66776 && raw_char <= 66811)
    || (raw_char >= 68800 && raw_char <= 68850)
    || (raw_char >= 71872 && raw_char <= 71903)
    || (raw_char >= 93792 && raw_char <= 93823)
    || (raw_char >= 119834 && raw_char <= 119859)
    || (raw_char >= 119886 && raw_char <= 119892)
    || (raw_char >= 119894 && raw_char <= 119911)
    || (raw_char >= 119938 && raw_char <= 119963)
    || (raw_char >= 119990 && raw_char <= 119993)
    || raw_char == 119995
    || (raw_char >= 119997 && raw_char <= 120003)
    || (raw_char >= 120005 && raw_char <= 120015)
    || (raw_char >= 120042 && raw_char <= 120067)
    || (raw_char >= 120094 && raw_char <= 120119)
    || (raw_char >= 120146 && raw_char <= 120171)
    || (raw_char >= 120198 && raw_char <= 120223)
    || (raw_char >= 120250 && raw_char <= 120275)
    || (raw_char >= 120302 && raw_char <= 120327)
    || (raw_char >= 120354 && raw_char <= 120379)
    || (raw_char >= 120406 && raw_char <= 120431)
    || (raw_char >= 120458 && raw_char <= 120485)
    || (raw_char >= 120514 && raw_char <= 120538)
    || (raw_char >= 120540 && raw_char <= 120545)
    || (raw_char >= 120572 && raw_char <= 120596)
    || (raw_char >= 120598 && raw_char <= 120603)
    || (raw_char >= 120630 && raw_char <= 120654)
    || (raw_char >= 120656 && raw_char <= 120661)
    || (raw_char >= 120688 && raw_char <= 120712)
    || (raw_char >= 120714 && raw_char <= 120719)
    || (raw_char >= 120746 && raw_char <= 120770)
    || (raw_char >= 120772 && raw_char <= 120777)
    || raw_char == 120779
    })
}

pub fn mn<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        (raw_char >= 768 && raw_char <= 879)
    || (raw_char >= 1155 && raw_char <= 1159)
    || (raw_char >= 1425 && raw_char <= 1469)
    || raw_char == 1471
    || (raw_char >= 1473 && raw_char <= 1474)
    || (raw_char >= 1476 && raw_char <= 1477)
    || raw_char == 1479
    || (raw_char >= 1552 && raw_char <= 1562)
    || (raw_char >= 1611 && raw_char <= 1631)
    || raw_char == 1648
    || (raw_char >= 1750 && raw_char <= 1756)
    || (raw_char >= 1759 && raw_char <= 1764)
    || (raw_char >= 1767 && raw_char <= 1768)
    || (raw_char >= 1770 && raw_char <= 1773)
    || raw_char == 1809
    || (raw_char >= 1840 && raw_char <= 1866)
    || (raw_char >= 1958 && raw_char <= 1968)
    || (raw_char >= 2027 && raw_char <= 2035)
    || raw_char == 2045
    || (raw_char >= 2070 && raw_char <= 2073)
    || (raw_char >= 2075 && raw_char <= 2083)
    || (raw_char >= 2085 && raw_char <= 2087)
    || (raw_char >= 2089 && raw_char <= 2093)
    || (raw_char >= 2137 && raw_char <= 2139)
    || (raw_char >= 2259 && raw_char <= 2273)
    || (raw_char >= 2275 && raw_char <= 2306)
    || raw_char == 2362
    || raw_char == 2364
    || (raw_char >= 2369 && raw_char <= 2376)
    || raw_char == 2381
    || (raw_char >= 2385 && raw_char <= 2391)
    || (raw_char >= 2402 && raw_char <= 2403)
    || raw_char == 2433
    || raw_char == 2492
    || (raw_char >= 2497 && raw_char <= 2500)
    || raw_char == 2509
    || (raw_char >= 2530 && raw_char <= 2531)
    || raw_char == 2558
    || (raw_char >= 2561 && raw_char <= 2562)
    || raw_char == 2620
    || (raw_char >= 2625 && raw_char <= 2626)
    || (raw_char >= 2631 && raw_char <= 2632)
    || (raw_char >= 2635 && raw_char <= 2637)
    || raw_char == 2641
    || (raw_char >= 2672 && raw_char <= 2673)
    || raw_char == 2677
    || (raw_char >= 2689 && raw_char <= 2690)
    || raw_char == 2748
    || (raw_char >= 2753 && raw_char <= 2757)
    || (raw_char >= 2759 && raw_char <= 2760)
    || raw_char == 2765
    || (raw_char >= 2786 && raw_char <= 2787)
    || (raw_char >= 2810 && raw_char <= 2815)
    || raw_char == 2817
    || raw_char == 2876
    || raw_char == 2879
    || (raw_char >= 2881 && raw_char <= 2884)
    || raw_char == 2893
    || raw_char == 2902
    || (raw_char >= 2914 && raw_char <= 2915)
    || raw_char == 2946
    || raw_char == 3008
    || raw_char == 3021
    || raw_char == 3072
    || raw_char == 3076
    || (raw_char >= 3134 && raw_char <= 3136)
    || (raw_char >= 3142 && raw_char <= 3144)
    || (raw_char >= 3146 && raw_char <= 3149)
    || (raw_char >= 3157 && raw_char <= 3158)
    || (raw_char >= 3170 && raw_char <= 3171)
    || raw_char == 3201
    || raw_char == 3260
    || raw_char == 3263
    || raw_char == 3270
    || (raw_char >= 3276 && raw_char <= 3277)
    || (raw_char >= 3298 && raw_char <= 3299)
    || (raw_char >= 3328 && raw_char <= 3329)
    || (raw_char >= 3387 && raw_char <= 3388)
    || (raw_char >= 3393 && raw_char <= 3396)
    || raw_char == 3405
    || (raw_char >= 3426 && raw_char <= 3427)
    || raw_char == 3530
    || (raw_char >= 3538 && raw_char <= 3540)
    || raw_char == 3542
    || raw_char == 3633
    || (raw_char >= 3636 && raw_char <= 3642)
    || (raw_char >= 3655 && raw_char <= 3662)
    || raw_char == 3761
    || (raw_char >= 3764 && raw_char <= 3769)
    || (raw_char >= 3771 && raw_char <= 3772)
    || (raw_char >= 3784 && raw_char <= 3789)
    || (raw_char >= 3864 && raw_char <= 3865)
    || raw_char == 3893
    || raw_char == 3895
    || raw_char == 3897
    || (raw_char >= 3953 && raw_char <= 3966)
    || (raw_char >= 3968 && raw_char <= 3972)
    || (raw_char >= 3974 && raw_char <= 3975)
    || (raw_char >= 3981 && raw_char <= 3991)
    || (raw_char >= 3993 && raw_char <= 4028)
    || raw_char == 4038
    || (raw_char >= 4141 && raw_char <= 4144)
    || (raw_char >= 4146 && raw_char <= 4151)
    || (raw_char >= 4153 && raw_char <= 4154)
    || (raw_char >= 4157 && raw_char <= 4158)
    || (raw_char >= 4184 && raw_char <= 4185)
    || (raw_char >= 4190 && raw_char <= 4192)
    || (raw_char >= 4209 && raw_char <= 4212)
    || raw_char == 4226
    || (raw_char >= 4229 && raw_char <= 4230)
    || raw_char == 4237
    || raw_char == 4253
    || (raw_char >= 4957 && raw_char <= 4959)
    || (raw_char >= 5906 && raw_char <= 5908)
    || (raw_char >= 5938 && raw_char <= 5940)
    || (raw_char >= 5970 && raw_char <= 5971)
    || (raw_char >= 6002 && raw_char <= 6003)
    || (raw_char >= 6068 && raw_char <= 6069)
    || (raw_char >= 6071 && raw_char <= 6077)
    || raw_char == 6086
    || (raw_char >= 6089 && raw_char <= 6099)
    || raw_char == 6109
    || (raw_char >= 6155 && raw_char <= 6157)
    || (raw_char >= 6277 && raw_char <= 6278)
    || raw_char == 6313
    || (raw_char >= 6432 && raw_char <= 6434)
    || (raw_char >= 6439 && raw_char <= 6440)
    || raw_char == 6450
    || (raw_char >= 6457 && raw_char <= 6459)
    || (raw_char >= 6679 && raw_char <= 6680)
    || raw_char == 6683
    || raw_char == 6742
    || (raw_char >= 6744 && raw_char <= 6750)
    || raw_char == 6752
    || raw_char == 6754
    || (raw_char >= 6757 && raw_char <= 6764)
    || (raw_char >= 6771 && raw_char <= 6780)
    || raw_char == 6783
    || (raw_char >= 6832 && raw_char <= 6845)
    || (raw_char >= 6912 && raw_char <= 6915)
    || raw_char == 6964
    || (raw_char >= 6966 && raw_char <= 6970)
    || raw_char == 6972
    || raw_char == 6978
    || (raw_char >= 7019 && raw_char <= 7027)
    || (raw_char >= 7040 && raw_char <= 7041)
    || (raw_char >= 7074 && raw_char <= 7077)
    || (raw_char >= 7080 && raw_char <= 7081)
    || (raw_char >= 7083 && raw_char <= 7085)
    || raw_char == 7142
    || (raw_char >= 7144 && raw_char <= 7145)
    || raw_char == 7149
    || (raw_char >= 7151 && raw_char <= 7153)
    || (raw_char >= 7212 && raw_char <= 7219)
    || (raw_char >= 7222 && raw_char <= 7223)
    || (raw_char >= 7376 && raw_char <= 7378)
    || (raw_char >= 7380 && raw_char <= 7392)
    || (raw_char >= 7394 && raw_char <= 7400)
    || raw_char == 7405
    || raw_char == 7412
    || (raw_char >= 7416 && raw_char <= 7417)
    || (raw_char >= 7616 && raw_char <= 7673)
    || (raw_char >= 7675 && raw_char <= 7679)
    || (raw_char >= 8400 && raw_char <= 8412)
    || raw_char == 8417
    || (raw_char >= 8421 && raw_char <= 8432)
    || (raw_char >= 11503 && raw_char <= 11505)
    || raw_char == 11647
    || (raw_char >= 11744 && raw_char <= 11775)
    || (raw_char >= 12330 && raw_char <= 12333)
    || (raw_char >= 12441 && raw_char <= 12442)
    || raw_char == 42607
    || (raw_char >= 42612 && raw_char <= 42621)
    || (raw_char >= 42654 && raw_char <= 42655)
    || (raw_char >= 42736 && raw_char <= 42737)
    || raw_char == 43010
    || raw_char == 43014
    || raw_char == 43019
    || (raw_char >= 43045 && raw_char <= 43046)
    || (raw_char >= 43204 && raw_char <= 43205)
    || (raw_char >= 43232 && raw_char <= 43249)
    || raw_char == 43263
    || (raw_char >= 43302 && raw_char <= 43309)
    || (raw_char >= 43335 && raw_char <= 43345)
    || (raw_char >= 43392 && raw_char <= 43394)
    || raw_char == 43443
    || (raw_char >= 43446 && raw_char <= 43449)
    || raw_char == 43452
    || raw_char == 43493
    || (raw_char >= 43561 && raw_char <= 43566)
    || (raw_char >= 43569 && raw_char <= 43570)
    || (raw_char >= 43573 && raw_char <= 43574)
    || raw_char == 43587
    || raw_char == 43596
    || raw_char == 43644
    || raw_char == 43696
    || (raw_char >= 43698 && raw_char <= 43700)
    || (raw_char >= 43703 && raw_char <= 43704)
    || (raw_char >= 43710 && raw_char <= 43711)
    || raw_char == 43713
    || (raw_char >= 43756 && raw_char <= 43757)
    || raw_char == 43766
    || raw_char == 44005
    || raw_char == 44008
    || raw_char == 44013
    || raw_char == 64286
    || (raw_char >= 65024 && raw_char <= 65039)
    || (raw_char >= 65056 && raw_char <= 65071)
    || raw_char == 66045
    || raw_char == 66272
    || (raw_char >= 66422 && raw_char <= 66426)
    || (raw_char >= 68097 && raw_char <= 68099)
    || (raw_char >= 68101 && raw_char <= 68102)
    || (raw_char >= 68108 && raw_char <= 68111)
    || (raw_char >= 68152 && raw_char <= 68154)
    || raw_char == 68159
    || (raw_char >= 68325 && raw_char <= 68326)
    || (raw_char >= 68900 && raw_char <= 68903)
    || (raw_char >= 69446 && raw_char <= 69456)
    || raw_char == 69633
    || (raw_char >= 69688 && raw_char <= 69702)
    || (raw_char >= 69759 && raw_char <= 69761)
    || (raw_char >= 69811 && raw_char <= 69814)
    || (raw_char >= 69817 && raw_char <= 69818)
    || (raw_char >= 69888 && raw_char <= 69890)
    || (raw_char >= 69927 && raw_char <= 69931)
    || (raw_char >= 69933 && raw_char <= 69940)
    || raw_char == 70003
    || (raw_char >= 70016 && raw_char <= 70017)
    || (raw_char >= 70070 && raw_char <= 70078)
    || (raw_char >= 70089 && raw_char <= 70092)
    || (raw_char >= 70191 && raw_char <= 70193)
    || raw_char == 70196
    || (raw_char >= 70198 && raw_char <= 70199)
    || raw_char == 70206
    || raw_char == 70367
    || (raw_char >= 70371 && raw_char <= 70378)
    || (raw_char >= 70400 && raw_char <= 70401)
    || (raw_char >= 70459 && raw_char <= 70460)
    || raw_char == 70464
    || (raw_char >= 70502 && raw_char <= 70508)
    || (raw_char >= 70512 && raw_char <= 70516)
    || (raw_char >= 70712 && raw_char <= 70719)
    || (raw_char >= 70722 && raw_char <= 70724)
    || raw_char == 70726
    || raw_char == 70750
    || (raw_char >= 70835 && raw_char <= 70840)
    || raw_char == 70842
    || (raw_char >= 70847 && raw_char <= 70848)
    || (raw_char >= 70850 && raw_char <= 70851)
    || (raw_char >= 71090 && raw_char <= 71093)
    || (raw_char >= 71100 && raw_char <= 71101)
    || (raw_char >= 71103 && raw_char <= 71104)
    || (raw_char >= 71132 && raw_char <= 71133)
    || (raw_char >= 71219 && raw_char <= 71226)
    || raw_char == 71229
    || (raw_char >= 71231 && raw_char <= 71232)
    || raw_char == 71339
    || raw_char == 71341
    || (raw_char >= 71344 && raw_char <= 71349)
    || raw_char == 71351
    || (raw_char >= 71453 && raw_char <= 71455)
    || (raw_char >= 71458 && raw_char <= 71461)
    || (raw_char >= 71463 && raw_char <= 71467)
    || (raw_char >= 71727 && raw_char <= 71735)
    || (raw_char >= 71737 && raw_char <= 71738)
    || (raw_char >= 72193 && raw_char <= 72202)
    || (raw_char >= 72243 && raw_char <= 72248)
    || (raw_char >= 72251 && raw_char <= 72254)
    || raw_char == 72263
    || (raw_char >= 72273 && raw_char <= 72278)
    || (raw_char >= 72281 && raw_char <= 72283)
    || (raw_char >= 72330 && raw_char <= 72342)
    || (raw_char >= 72344 && raw_char <= 72345)
    || (raw_char >= 72752 && raw_char <= 72758)
    || (raw_char >= 72760 && raw_char <= 72765)
    || raw_char == 72767
    || (raw_char >= 72850 && raw_char <= 72871)
    || (raw_char >= 72874 && raw_char <= 72880)
    || (raw_char >= 72882 && raw_char <= 72883)
    || (raw_char >= 72885 && raw_char <= 72886)
    || (raw_char >= 73009 && raw_char <= 73014)
    || raw_char == 73018
    || (raw_char >= 73020 && raw_char <= 73021)
    || (raw_char >= 73023 && raw_char <= 73029)
    || raw_char == 73031
    || (raw_char >= 73104 && raw_char <= 73105)
    || raw_char == 73109
    || raw_char == 73111
    || (raw_char >= 73459 && raw_char <= 73460)
    || (raw_char >= 92912 && raw_char <= 92916)
    || (raw_char >= 92976 && raw_char <= 92982)
    || (raw_char >= 94095 && raw_char <= 94098)
    || (raw_char >= 113821 && raw_char <= 113822)
    || (raw_char >= 119143 && raw_char <= 119145)
    || (raw_char >= 119163 && raw_char <= 119170)
    || (raw_char >= 119173 && raw_char <= 119179)
    || (raw_char >= 119210 && raw_char <= 119213)
    || (raw_char >= 119362 && raw_char <= 119364)
    || (raw_char >= 121344 && raw_char <= 121398)
    || (raw_char >= 121403 && raw_char <= 121452)
    || raw_char == 121461
    || raw_char == 121476
    || (raw_char >= 121499 && raw_char <= 121503)
    || (raw_char >= 121505 && raw_char <= 121519)
    || (raw_char >= 122880 && raw_char <= 122886)
    || (raw_char >= 122888 && raw_char <= 122904)
    || (raw_char >= 122907 && raw_char <= 122913)
    || (raw_char >= 122915 && raw_char <= 122916)
    || (raw_char >= 122918 && raw_char <= 122922)
    || (raw_char >= 125136 && raw_char <= 125142)
    || (raw_char >= 125252 && raw_char <= 125258)
    })
}

pub fn mc<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
    raw_char == 2307
    || raw_char == 2363
    || (raw_char >= 2366 && raw_char <= 2368)
    || (raw_char >= 2377 && raw_char <= 2380)
    || (raw_char >= 2382 && raw_char <= 2383)
    || (raw_char >= 2434 && raw_char <= 2435)
    || (raw_char >= 2494 && raw_char <= 2496)
    || (raw_char >= 2503 && raw_char <= 2504)
    || (raw_char >= 2507 && raw_char <= 2508)
    || raw_char == 2519
    || raw_char == 2563
    || (raw_char >= 2622 && raw_char <= 2624)
    || raw_char == 2691
    || (raw_char >= 2750 && raw_char <= 2752)
    || raw_char == 2761
    || (raw_char >= 2763 && raw_char <= 2764)
    || (raw_char >= 2818 && raw_char <= 2819)
    || raw_char == 2878
    || raw_char == 2880
    || (raw_char >= 2887 && raw_char <= 2888)
    || (raw_char >= 2891 && raw_char <= 2892)
    || raw_char == 2903
    || (raw_char >= 3006 && raw_char <= 3007)
    || (raw_char >= 3009 && raw_char <= 3010)
    || (raw_char >= 3014 && raw_char <= 3016)
    || (raw_char >= 3018 && raw_char <= 3020)
    || raw_char == 3031
    || (raw_char >= 3073 && raw_char <= 3075)
    || (raw_char >= 3137 && raw_char <= 3140)
    || (raw_char >= 3202 && raw_char <= 3203)
    || raw_char == 3262
    || (raw_char >= 3264 && raw_char <= 3268)
    || (raw_char >= 3271 && raw_char <= 3272)
    || (raw_char >= 3274 && raw_char <= 3275)
    || (raw_char >= 3285 && raw_char <= 3286)
    || (raw_char >= 3330 && raw_char <= 3331)
    || (raw_char >= 3390 && raw_char <= 3392)
    || (raw_char >= 3398 && raw_char <= 3400)
    || (raw_char >= 3402 && raw_char <= 3404)
    || raw_char == 3415
    || (raw_char >= 3458 && raw_char <= 3459)
    || (raw_char >= 3535 && raw_char <= 3537)
    || (raw_char >= 3544 && raw_char <= 3551)
    || (raw_char >= 3570 && raw_char <= 3571)
    || (raw_char >= 3902 && raw_char <= 3903)
    || raw_char == 3967
    || (raw_char >= 4139 && raw_char <= 4140)
    || raw_char == 4145
    || raw_char == 4152
    || (raw_char >= 4155 && raw_char <= 4156)
    || (raw_char >= 4182 && raw_char <= 4183)
    || (raw_char >= 4194 && raw_char <= 4196)
    || (raw_char >= 4199 && raw_char <= 4205)
    || (raw_char >= 4227 && raw_char <= 4228)
    || (raw_char >= 4231 && raw_char <= 4236)
    || raw_char == 4239
    || (raw_char >= 4250 && raw_char <= 4252)
    || raw_char == 6070
    || (raw_char >= 6078 && raw_char <= 6085)
    || (raw_char >= 6087 && raw_char <= 6088)
    || (raw_char >= 6435 && raw_char <= 6438)
    || (raw_char >= 6441 && raw_char <= 6443)
    || (raw_char >= 6448 && raw_char <= 6449)
    || (raw_char >= 6451 && raw_char <= 6456)
    || (raw_char >= 6681 && raw_char <= 6682)
    || raw_char == 6741
    || raw_char == 6743
    || raw_char == 6753
    || (raw_char >= 6755 && raw_char <= 6756)
    || (raw_char >= 6765 && raw_char <= 6770)
    || raw_char == 6916
    || raw_char == 6965
    || raw_char == 6971
    || (raw_char >= 6973 && raw_char <= 6977)
    || (raw_char >= 6979 && raw_char <= 6980)
    || raw_char == 7042
    || raw_char == 7073
    || (raw_char >= 7078 && raw_char <= 7079)
    || raw_char == 7082
    || raw_char == 7143
    || (raw_char >= 7146 && raw_char <= 7148)
    || raw_char == 7150
    || (raw_char >= 7154 && raw_char <= 7155)
    || (raw_char >= 7204 && raw_char <= 7211)
    || (raw_char >= 7220 && raw_char <= 7221)
    || raw_char == 7393
    || (raw_char >= 7410 && raw_char <= 7411)
    || raw_char == 7415
    || (raw_char >= 12334 && raw_char <= 12335)
    || (raw_char >= 43043 && raw_char <= 43044)
    || raw_char == 43047
    || (raw_char >= 43136 && raw_char <= 43137)
    || (raw_char >= 43188 && raw_char <= 43203)
    || (raw_char >= 43346 && raw_char <= 43347)
    || raw_char == 43395
    || (raw_char >= 43444 && raw_char <= 43445)
    || (raw_char >= 43450 && raw_char <= 43451)
    || (raw_char >= 43453 && raw_char <= 43456)
    || (raw_char >= 43567 && raw_char <= 43568)
    || (raw_char >= 43571 && raw_char <= 43572)
    || raw_char == 43597
    || raw_char == 43643
    || raw_char == 43645
    || raw_char == 43755
    || (raw_char >= 43758 && raw_char <= 43759)
    || raw_char == 43765
    || (raw_char >= 44003 && raw_char <= 44004)
    || (raw_char >= 44006 && raw_char <= 44007)
    || (raw_char >= 44009 && raw_char <= 44010)
    || raw_char == 44012
    || raw_char == 69632
    || raw_char == 69634
    || raw_char == 69762
    || (raw_char >= 69808 && raw_char <= 69810)
    || (raw_char >= 69815 && raw_char <= 69816)
    || raw_char == 69932
    || (raw_char >= 69957 && raw_char <= 69958)
    || raw_char == 70018
    || (raw_char >= 70067 && raw_char <= 70069)
    || (raw_char >= 70079 && raw_char <= 70080)
    || (raw_char >= 70188 && raw_char <= 70190)
    || (raw_char >= 70194 && raw_char <= 70195)
    || raw_char == 70197
    || (raw_char >= 70368 && raw_char <= 70370)
    || (raw_char >= 70402 && raw_char <= 70403)
    || (raw_char >= 70462 && raw_char <= 70463)
    || (raw_char >= 70465 && raw_char <= 70468)
    || (raw_char >= 70471 && raw_char <= 70472)
    || (raw_char >= 70475 && raw_char <= 70477)
    || raw_char == 70487
    || (raw_char >= 70498 && raw_char <= 70499)
    || (raw_char >= 70709 && raw_char <= 70711)
    || (raw_char >= 70720 && raw_char <= 70721)
    || raw_char == 70725
    || (raw_char >= 70832 && raw_char <= 70834)
    || raw_char == 70841
    || (raw_char >= 70843 && raw_char <= 70846)
    || raw_char == 70849
    || (raw_char >= 71087 && raw_char <= 71089)
    || (raw_char >= 71096 && raw_char <= 71099)
    || raw_char == 71102
    || (raw_char >= 71216 && raw_char <= 71218)
    || (raw_char >= 71227 && raw_char <= 71228)
    || raw_char == 71230
    || raw_char == 71340
    || (raw_char >= 71342 && raw_char <= 71343)
    || raw_char == 71350
    || (raw_char >= 71456 && raw_char <= 71457)
    || raw_char == 71462
    || (raw_char >= 71724 && raw_char <= 71726)
    || raw_char == 71736
    || raw_char == 72249
    || (raw_char >= 72279 && raw_char <= 72280)
    || raw_char == 72343
    || raw_char == 72751
    || raw_char == 72766
    || raw_char == 72873
    || raw_char == 72881
    || raw_char == 72884
    || (raw_char >= 73098 && raw_char <= 73102)
    || (raw_char >= 73107 && raw_char <= 73108)
    || raw_char == 73110
    || (raw_char >= 73461 && raw_char <= 73462)
    || (raw_char >= 94033 && raw_char <= 94078)
    || (raw_char >= 119141 && raw_char <= 119142)
    })
}

pub fn lu<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        (raw_char >= 65 && raw_char <= 90)
        || (raw_char >= 192 && raw_char <= 214)
        || (raw_char >= 216 && raw_char <= 222)
        || raw_char == 256
        || raw_char == 258
        || raw_char == 260
        || raw_char == 262
        || raw_char == 264
        || raw_char == 266
        || raw_char == 268
        || raw_char == 270
        || raw_char == 272
        || raw_char == 274
        || raw_char == 276
        || raw_char == 278
        || raw_char == 280
        || raw_char == 282
        || raw_char == 284
        || raw_char == 286
        || raw_char == 288
        || raw_char == 290
        || raw_char == 292
        || raw_char == 294
        || raw_char == 296
        || raw_char == 298
        || raw_char == 300
        || raw_char == 302
        || raw_char == 304
        || raw_char == 306
        || raw_char == 308
        || raw_char == 310
        || raw_char == 313
        || raw_char == 315
        || raw_char == 317
        || raw_char == 319
        || raw_char == 321
        || raw_char == 323
        || raw_char == 325
        || raw_char == 327
        || raw_char == 330
        || raw_char == 332
        || raw_char == 334
        || raw_char == 336
        || raw_char == 338
        || raw_char == 340
        || raw_char == 342
        || raw_char == 344
        || raw_char == 346
        || raw_char == 348
        || raw_char == 350
        || raw_char == 352
        || raw_char == 354
        || raw_char == 356
        || raw_char == 358
        || raw_char == 360
        || raw_char == 362
        || raw_char == 364
        || raw_char == 366
        || raw_char == 368
        || raw_char == 370
        || raw_char == 372
        || raw_char == 374
        || (raw_char >= 376 && raw_char <= 377)
        || raw_char == 379
        || raw_char == 381
        || (raw_char >= 385 && raw_char <= 386)
        || raw_char == 388
        || (raw_char >= 390 && raw_char <= 391)
        || (raw_char >= 393 && raw_char <= 395)
        || (raw_char >= 398 && raw_char <= 401)
        || (raw_char >= 403 && raw_char <= 404)
        || (raw_char >= 406 && raw_char <= 408)
        || (raw_char >= 412 && raw_char <= 413)
        || (raw_char >= 415 && raw_char <= 416)
        || raw_char == 418
        || raw_char == 420
        || (raw_char >= 422 && raw_char <= 423)
        || raw_char == 425
        || raw_char == 428
        || (raw_char >= 430 && raw_char <= 431)
        || (raw_char >= 433 && raw_char <= 435)
        || raw_char == 437
        || (raw_char >= 439 && raw_char <= 440)
        || raw_char == 444
        || raw_char == 452
        || raw_char == 455
        || raw_char == 458
        || raw_char == 461
        || raw_char == 463
        || raw_char == 465
        || raw_char == 467
        || raw_char == 469
        || raw_char == 471
        || raw_char == 473
        || raw_char == 475
        || raw_char == 478
        || raw_char == 480
        || raw_char == 482
        || raw_char == 484
        || raw_char == 486
        || raw_char == 488
        || raw_char == 490
        || raw_char == 492
        || raw_char == 494
        || raw_char == 497
        || raw_char == 500
        || (raw_char >= 502 && raw_char <= 504)
        || raw_char == 506
        || raw_char == 508
        || raw_char == 510
        || raw_char == 512
        || raw_char == 514
        || raw_char == 516
        || raw_char == 518
        || raw_char == 520
        || raw_char == 522
        || raw_char == 524
        || raw_char == 526
        || raw_char == 528
        || raw_char == 530
        || raw_char == 532
        || raw_char == 534
        || raw_char == 536
        || raw_char == 538
        || raw_char == 540
        || raw_char == 542
        || raw_char == 544
        || raw_char == 546
        || raw_char == 548
        || raw_char == 550
        || raw_char == 552
        || raw_char == 554
        || raw_char == 556
        || raw_char == 558
        || raw_char == 560
        || raw_char == 562
        || (raw_char >= 570 && raw_char <= 571)
        || (raw_char >= 573 && raw_char <= 574)
        || raw_char == 577
        || (raw_char >= 579 && raw_char <= 582)
        || raw_char == 584
        || raw_char == 586
        || raw_char == 588
        || raw_char == 590
        || raw_char == 880
        || raw_char == 882
        || raw_char == 886
        || raw_char == 895
        || raw_char == 902
        || (raw_char >= 904 && raw_char <= 906)
        || raw_char == 908
        || (raw_char >= 910 && raw_char <= 911)
        || (raw_char >= 913 && raw_char <= 929)
        || (raw_char >= 931 && raw_char <= 939)
        || raw_char == 975
        || (raw_char >= 978 && raw_char <= 980)
        || raw_char == 984
        || raw_char == 986
        || raw_char == 988
        || raw_char == 990
        || raw_char == 992
        || raw_char == 994
        || raw_char == 996
        || raw_char == 998
        || raw_char == 1000
        || raw_char == 1002
        || raw_char == 1004
        || raw_char == 1006
        || raw_char == 1012
        || raw_char == 1015
        || (raw_char >= 1017 && raw_char <= 1018)
        || (raw_char >= 1021 && raw_char <= 1071)
        || raw_char == 1120
        || raw_char == 1122
        || raw_char == 1124
        || raw_char == 1126
        || raw_char == 1128
        || raw_char == 1130
        || raw_char == 1132
        || raw_char == 1134
        || raw_char == 1136
        || raw_char == 1138
        || raw_char == 1140
        || raw_char == 1142
        || raw_char == 1144
        || raw_char == 1146
        || raw_char == 1148
        || raw_char == 1150
        || raw_char == 1152
        || raw_char == 1162
        || raw_char == 1164
        || raw_char == 1166
        || raw_char == 1168
        || raw_char == 1170
        || raw_char == 1172
        || raw_char == 1174
        || raw_char == 1176
        || raw_char == 1178
        || raw_char == 1180
        || raw_char == 1182
        || raw_char == 1184
        || raw_char == 1186
        || raw_char == 1188
        || raw_char == 1190
        || raw_char == 1192
        || raw_char == 1194
        || raw_char == 1196
        || raw_char == 1198
        || raw_char == 1200
        || raw_char == 1202
        || raw_char == 1204
        || raw_char == 1206
        || raw_char == 1208
        || raw_char == 1210
        || raw_char == 1212
        || raw_char == 1214
        || (raw_char >= 1216 && raw_char <= 1217)
        || raw_char == 1219
        || raw_char == 1221
        || raw_char == 1223
        || raw_char == 1225
        || raw_char == 1227
        || raw_char == 1229
        || raw_char == 1232
        || raw_char == 1234
        || raw_char == 1236
        || raw_char == 1238
        || raw_char == 1240
        || raw_char == 1242
        || raw_char == 1244
        || raw_char == 1246
        || raw_char == 1248
        || raw_char == 1250
        || raw_char == 1252
        || raw_char == 1254
        || raw_char == 1256
        || raw_char == 1258
        || raw_char == 1260
        || raw_char == 1262
        || raw_char == 1264
        || raw_char == 1266
        || raw_char == 1268
        || raw_char == 1270
        || raw_char == 1272
        || raw_char == 1274
        || raw_char == 1276
        || raw_char == 1278
        || raw_char == 1280
        || raw_char == 1282
        || raw_char == 1284
        || raw_char == 1286
        || raw_char == 1288
        || raw_char == 1290
        || raw_char == 1292
        || raw_char == 1294
        || raw_char == 1296
        || raw_char == 1298
        || raw_char == 1300
        || raw_char == 1302
        || raw_char == 1304
        || raw_char == 1306
        || raw_char == 1308
        || raw_char == 1310
        || raw_char == 1312
        || raw_char == 1314
        || raw_char == 1316
        || raw_char == 1318
        || raw_char == 1320
        || raw_char == 1322
        || raw_char == 1324
        || raw_char == 1326
        || (raw_char >= 1329 && raw_char <= 1366)
        || (raw_char >= 4256 && raw_char <= 4293)
        || raw_char == 4295
        || raw_char == 4301
        || (raw_char >= 5024 && raw_char <= 5109)
        || (raw_char >= 7312 && raw_char <= 7354)
        || (raw_char >= 7357 && raw_char <= 7359)
        || raw_char == 7680
        || raw_char == 7682
        || raw_char == 7684
        || raw_char == 7686
        || raw_char == 7688
        || raw_char == 7690
        || raw_char == 7692
        || raw_char == 7694
        || raw_char == 7696
        || raw_char == 7698
        || raw_char == 7700
        || raw_char == 7702
        || raw_char == 7704
        || raw_char == 7706
        || raw_char == 7708
        || raw_char == 7710
        || raw_char == 7712
        || raw_char == 7714
        || raw_char == 7716
        || raw_char == 7718
        || raw_char == 7720
        || raw_char == 7722
        || raw_char == 7724
        || raw_char == 7726
        || raw_char == 7728
        || raw_char == 7730
        || raw_char == 7732
        || raw_char == 7734
        || raw_char == 7736
        || raw_char == 7738
        || raw_char == 7740
        || raw_char == 7742
        || raw_char == 7744
        || raw_char == 7746
        || raw_char == 7748
        || raw_char == 7750
        || raw_char == 7752
        || raw_char == 7754
        || raw_char == 7756
        || raw_char == 7758
        || raw_char == 7760
        || raw_char == 7762
        || raw_char == 7764
        || raw_char == 7766
        || raw_char == 7768
        || raw_char == 7770
        || raw_char == 7772
        || raw_char == 7774
        || raw_char == 7776
        || raw_char == 7778
        || raw_char == 7780
        || raw_char == 7782
        || raw_char == 7784
        || raw_char == 7786
        || raw_char == 7788
        || raw_char == 7790
        || raw_char == 7792
        || raw_char == 7794
        || raw_char == 7796
        || raw_char == 7798
        || raw_char == 7800
        || raw_char == 7802
        || raw_char == 7804
        || raw_char == 7806
        || raw_char == 7808
        || raw_char == 7810
        || raw_char == 7812
        || raw_char == 7814
        || raw_char == 7816
        || raw_char == 7818
        || raw_char == 7820
        || raw_char == 7822
        || raw_char == 7824
        || raw_char == 7826
        || raw_char == 7828
        || raw_char == 7838
        || raw_char == 7840
        || raw_char == 7842
        || raw_char == 7844
        || raw_char == 7846
        || raw_char == 7848
        || raw_char == 7850
        || raw_char == 7852
        || raw_char == 7854
        || raw_char == 7856
        || raw_char == 7858
        || raw_char == 7860
        || raw_char == 7862
        || raw_char == 7864
        || raw_char == 7866
        || raw_char == 7868
        || raw_char == 7870
        || raw_char == 7872
        || raw_char == 7874
        || raw_char == 7876
        || raw_char == 7878
        || raw_char == 7880
        || raw_char == 7882
        || raw_char == 7884
        || raw_char == 7886
        || raw_char == 7888
        || raw_char == 7890
        || raw_char == 7892
        || raw_char == 7894
        || raw_char == 7896
        || raw_char == 7898
        || raw_char == 7900
        || raw_char == 7902
        || raw_char == 7904
        || raw_char == 7906
        || raw_char == 7908
        || raw_char == 7910
        || raw_char == 7912
        || raw_char == 7914
        || raw_char == 7916
        || raw_char == 7918
        || raw_char == 7920
        || raw_char == 7922
        || raw_char == 7924
        || raw_char == 7926
        || raw_char == 7928
        || raw_char == 7930
        || raw_char == 7932
        || raw_char == 7934
        || (raw_char >= 7944 && raw_char <= 7951)
        || (raw_char >= 7960 && raw_char <= 7965)
        || (raw_char >= 7976 && raw_char <= 7983)
        || (raw_char >= 7992 && raw_char <= 7999)
        || (raw_char >= 8008 && raw_char <= 8013)
        || raw_char == 8025
        || raw_char == 8027
        || raw_char == 8029
        || raw_char == 8031
        || (raw_char >= 8040 && raw_char <= 8047)
        || (raw_char >= 8120 && raw_char <= 8123)
        || (raw_char >= 8136 && raw_char <= 8139)
        || (raw_char >= 8152 && raw_char <= 8155)
        || (raw_char >= 8168 && raw_char <= 8172)
        || (raw_char >= 8184 && raw_char <= 8187)
        || raw_char == 8450
        || raw_char == 8455
        || (raw_char >= 8459 && raw_char <= 8461)
        || (raw_char >= 8464 && raw_char <= 8466)
        || raw_char == 8469
        || (raw_char >= 8473 && raw_char <= 8477)
        || raw_char == 8484
        || raw_char == 8486
        || raw_char == 8488
        || (raw_char >= 8490 && raw_char <= 8493)
        || (raw_char >= 8496 && raw_char <= 8499)
        || (raw_char >= 8510 && raw_char <= 8511)
        || raw_char == 8517
        || raw_char == 8579
        || (raw_char >= 11264 && raw_char <= 11310)
        || raw_char == 11360
        || (raw_char >= 11362 && raw_char <= 11364)
        || raw_char == 11367
        || raw_char == 11369
        || raw_char == 11371
        || (raw_char >= 11373 && raw_char <= 11376)
        || raw_char == 11378
        || raw_char == 11381
        || (raw_char >= 11390 && raw_char <= 11392)
        || raw_char == 11394
        || raw_char == 11396
        || raw_char == 11398
        || raw_char == 11400
        || raw_char == 11402
        || raw_char == 11404
        || raw_char == 11406
        || raw_char == 11408
        || raw_char == 11410
        || raw_char == 11412
        || raw_char == 11414
        || raw_char == 11416
        || raw_char == 11418
        || raw_char == 11420
        || raw_char == 11422
        || raw_char == 11424
        || raw_char == 11426
        || raw_char == 11428
        || raw_char == 11430
        || raw_char == 11432
        || raw_char == 11434
        || raw_char == 11436
        || raw_char == 11438
        || raw_char == 11440
        || raw_char == 11442
        || raw_char == 11444
        || raw_char == 11446
        || raw_char == 11448
        || raw_char == 11450
        || raw_char == 11452
        || raw_char == 11454
        || raw_char == 11456
        || raw_char == 11458
        || raw_char == 11460
        || raw_char == 11462
        || raw_char == 11464
        || raw_char == 11466
        || raw_char == 11468
        || raw_char == 11470
        || raw_char == 11472
        || raw_char == 11474
        || raw_char == 11476
        || raw_char == 11478
        || raw_char == 11480
        || raw_char == 11482
        || raw_char == 11484
        || raw_char == 11486
        || raw_char == 11488
        || raw_char == 11490
        || raw_char == 11499
        || raw_char == 11501
        || raw_char == 11506
        || raw_char == 42560
        || raw_char == 42562
        || raw_char == 42564
        || raw_char == 42566
        || raw_char == 42568
        || raw_char == 42570
        || raw_char == 42572
        || raw_char == 42574
        || raw_char == 42576
        || raw_char == 42578
        || raw_char == 42580
        || raw_char == 42582
        || raw_char == 42584
        || raw_char == 42586
        || raw_char == 42588
        || raw_char == 42590
        || raw_char == 42592
        || raw_char == 42594
        || raw_char == 42596
        || raw_char == 42598
        || raw_char == 42600
        || raw_char == 42602
        || raw_char == 42604
        || raw_char == 42624
        || raw_char == 42626
        || raw_char == 42628
        || raw_char == 42630
        || raw_char == 42632
        || raw_char == 42634
        || raw_char == 42636
        || raw_char == 42638
        || raw_char == 42640
        || raw_char == 42642
        || raw_char == 42644
        || raw_char == 42646
        || raw_char == 42648
        || raw_char == 42650
        || raw_char == 42786
        || raw_char == 42788
        || raw_char == 42790
        || raw_char == 42792
        || raw_char == 42794
        || raw_char == 42796
        || raw_char == 42798
        || raw_char == 42802
        || raw_char == 42804
        || raw_char == 42806
        || raw_char == 42808
        || raw_char == 42810
        || raw_char == 42812
        || raw_char == 42814
        || raw_char == 42816
        || raw_char == 42818
        || raw_char == 42820
        || raw_char == 42822
        || raw_char == 42824
        || raw_char == 42826
        || raw_char == 42828
        || raw_char == 42830
        || raw_char == 42832
        || raw_char == 42834
        || raw_char == 42836
        || raw_char == 42838
        || raw_char == 42840
        || raw_char == 42842
        || raw_char == 42844
        || raw_char == 42846
        || raw_char == 42848
        || raw_char == 42850
        || raw_char == 42852
        || raw_char == 42854
        || raw_char == 42856
        || raw_char == 42858
        || raw_char == 42860
        || raw_char == 42862
        || raw_char == 42873
        || raw_char == 42875
        || (raw_char >= 42877 && raw_char <= 42878)
        || raw_char == 42880
        || raw_char == 42882
        || raw_char == 42884
        || raw_char == 42886
        || raw_char == 42891
        || raw_char == 42893
        || raw_char == 42896
        || raw_char == 42898
        || raw_char == 42902
        || raw_char == 42904
        || raw_char == 42906
        || raw_char == 42908
        || raw_char == 42910
        || raw_char == 42912
        || raw_char == 42914
        || raw_char == 42916
        || raw_char == 42918
        || raw_char == 42920
        || (raw_char >= 42922 && raw_char <= 42926)
        || (raw_char >= 42928 && raw_char <= 42932)
        || raw_char == 42934
        || raw_char == 42936
        || (raw_char >= 65313 && raw_char <= 65338)
        || (raw_char >= 66560 && raw_char <= 66599)
        || (raw_char >= 66736 && raw_char <= 66771)
        || (raw_char >= 68736 && raw_char <= 68786)
        || (raw_char >= 71840 && raw_char <= 71871)
        || (raw_char >= 93760 && raw_char <= 93791)
        || (raw_char >= 119808 && raw_char <= 119833)
        || (raw_char >= 119860 && raw_char <= 119885)
        || (raw_char >= 119912 && raw_char <= 119937)
        || raw_char == 119964
        || (raw_char >= 119966 && raw_char <= 119967)
        || raw_char == 119970
        || (raw_char >= 119973 && raw_char <= 119974)
        || (raw_char >= 119977 && raw_char <= 119980)
        || (raw_char >= 119982 && raw_char <= 119989)
        || (raw_char >= 120016 && raw_char <= 120041)
        || (raw_char >= 120068 && raw_char <= 120069)
        || (raw_char >= 120071 && raw_char <= 120074)
        || (raw_char >= 120077 && raw_char <= 120084)
        || (raw_char >= 120086 && raw_char <= 120092)
        || (raw_char >= 120120 && raw_char <= 120121)
        || (raw_char >= 120123 && raw_char <= 120126)
        || (raw_char >= 120128 && raw_char <= 120132)
        || raw_char == 120134
        || (raw_char >= 120138 && raw_char <= 120144)
        || (raw_char >= 120172 && raw_char <= 120197)
        || (raw_char >= 120224 && raw_char <= 120249)
        || (raw_char >= 120276 && raw_char <= 120301)
        || (raw_char >= 120328 && raw_char <= 120353)
        || (raw_char >= 120380 && raw_char <= 120405)
        || (raw_char >= 120432 && raw_char <= 120457)
        || (raw_char >= 120488 && raw_char <= 120512)
        || (raw_char >= 120546 && raw_char <= 120570)
        || (raw_char >= 120604 && raw_char <= 120628)
        || (raw_char >= 120662 && raw_char <= 120686)
        || (raw_char >= 120720 && raw_char <= 120744)
        || raw_char == 120778
    })
}

pub fn lt<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        raw_char == 453
        || raw_char == 456
        || raw_char == 459
        || raw_char == 498
        || (raw_char >= 8072 && raw_char <= 8079)
        || (raw_char >= 8088 && raw_char <= 8095)
        || (raw_char >= 8104 && raw_char <= 8111)
        || raw_char == 8124
        || raw_char == 8140
    })
}

pub fn lm<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        (raw_char >= 688 && raw_char <= 705)
        || (raw_char >= 710 && raw_char <= 721)
        || (raw_char >= 736 && raw_char <= 740)
        || raw_char == 748
        || raw_char == 750
        || raw_char == 884
        || raw_char == 890
        || raw_char == 1369
        || raw_char == 1600
        || (raw_char >= 1765 && raw_char <= 1766)
        || (raw_char >= 2036 && raw_char <= 2037)
        || raw_char == 2042
        || raw_char == 2074
        || raw_char == 2084
        || raw_char == 2088
        || raw_char == 2417
        || raw_char == 3654
        || raw_char == 3782
        || raw_char == 4348
        || raw_char == 6103
        || raw_char == 6211
        || raw_char == 6823
        || (raw_char >= 7288 && raw_char <= 7293)
        || (raw_char >= 7468 && raw_char <= 7530)
        || raw_char == 7544
        || (raw_char >= 7579 && raw_char <= 7615)
        || raw_char == 8305
        || raw_char == 8319
        || (raw_char >= 8336 && raw_char <= 8348)
        || (raw_char >= 11388 && raw_char <= 11389)
        || raw_char == 11631
        || raw_char == 11823
        || raw_char == 12293
        || (raw_char >= 12337 && raw_char <= 12341)
        || raw_char == 12347
        || (raw_char >= 12445 && raw_char <= 12446)
        || (raw_char >= 12540 && raw_char <= 12542)
        || raw_char == 40981
        || (raw_char >= 42232 && raw_char <= 42237)
        || raw_char == 42508
        || raw_char == 42623
        || (raw_char >= 42652 && raw_char <= 42653)
        || (raw_char >= 42775 && raw_char <= 42783)
        || raw_char == 42864
        || raw_char == 42888
        || (raw_char >= 43000 && raw_char <= 43001)
        || raw_char == 43471
        || raw_char == 43494
        || raw_char == 43632
        || raw_char == 43741
        || (raw_char >= 43763 && raw_char <= 43764)
        || (raw_char >= 43868 && raw_char <= 43871)
        || raw_char == 65392
        || (raw_char >= 65438 && raw_char <= 65439)
        || (raw_char >= 92992 && raw_char <= 92995)
        || (raw_char >= 94099 && raw_char <= 94111)
    })
}

pub fn lo<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        raw_char == 170
        || raw_char == 186
        || raw_char == 443
        || (raw_char >= 448 && raw_char <= 451)
        || raw_char == 660
        || (raw_char >= 1488 && raw_char <= 1514)
        || (raw_char >= 1519 && raw_char <= 1522)
        || (raw_char >= 1568 && raw_char <= 1599)
        || (raw_char >= 1601 && raw_char <= 1610)
        || (raw_char >= 1646 && raw_char <= 1647)
        || (raw_char >= 1649 && raw_char <= 1747)
        || raw_char == 1749
        || (raw_char >= 1774 && raw_char <= 1775)
        || (raw_char >= 1786 && raw_char <= 1788)
        || raw_char == 1791
        || raw_char == 1808
        || (raw_char >= 1810 && raw_char <= 1839)
        || (raw_char >= 1869 && raw_char <= 1957)
        || raw_char == 1969
        || (raw_char >= 1994 && raw_char <= 2026)
        || (raw_char >= 2048 && raw_char <= 2069)
        || (raw_char >= 2112 && raw_char <= 2136)
        || (raw_char >= 2144 && raw_char <= 2154)
        || (raw_char >= 2208 && raw_char <= 2228)
        || (raw_char >= 2230 && raw_char <= 2237)
        || (raw_char >= 2308 && raw_char <= 2361)
        || raw_char == 2365
        || raw_char == 2384
        || (raw_char >= 2392 && raw_char <= 2401)
        || (raw_char >= 2418 && raw_char <= 2432)
        || (raw_char >= 2437 && raw_char <= 2444)
        || (raw_char >= 2447 && raw_char <= 2448)
        || (raw_char >= 2451 && raw_char <= 2472)
        || (raw_char >= 2474 && raw_char <= 2480)
        || raw_char == 2482
        || (raw_char >= 2486 && raw_char <= 2489)
        || raw_char == 2493
        || raw_char == 2510
        || (raw_char >= 2524 && raw_char <= 2525)
        || (raw_char >= 2527 && raw_char <= 2529)
        || (raw_char >= 2544 && raw_char <= 2545)
        || raw_char == 2556
        || (raw_char >= 2565 && raw_char <= 2570)
        || (raw_char >= 2575 && raw_char <= 2576)
        || (raw_char >= 2579 && raw_char <= 2600)
        || (raw_char >= 2602 && raw_char <= 2608)
        || (raw_char >= 2610 && raw_char <= 2611)
        || (raw_char >= 2613 && raw_char <= 2614)
        || (raw_char >= 2616 && raw_char <= 2617)
        || (raw_char >= 2649 && raw_char <= 2652)
        || raw_char == 2654
        || (raw_char >= 2674 && raw_char <= 2676)
        || (raw_char >= 2693 && raw_char <= 2701)
        || (raw_char >= 2703 && raw_char <= 2705)
        || (raw_char >= 2707 && raw_char <= 2728)
        || (raw_char >= 2730 && raw_char <= 2736)
        || (raw_char >= 2738 && raw_char <= 2739)
        || (raw_char >= 2741 && raw_char <= 2745)
        || raw_char == 2749
        || raw_char == 2768
        || (raw_char >= 2784 && raw_char <= 2785)
        || raw_char == 2809
        || (raw_char >= 2821 && raw_char <= 2828)
        || (raw_char >= 2831 && raw_char <= 2832)
        || (raw_char >= 2835 && raw_char <= 2856)
        || (raw_char >= 2858 && raw_char <= 2864)
        || (raw_char >= 2866 && raw_char <= 2867)
        || (raw_char >= 2869 && raw_char <= 2873)
        || raw_char == 2877
        || (raw_char >= 2908 && raw_char <= 2909)
        || (raw_char >= 2911 && raw_char <= 2913)
        || raw_char == 2929
        || raw_char == 2947
        || (raw_char >= 2949 && raw_char <= 2954)
        || (raw_char >= 2958 && raw_char <= 2960)
        || (raw_char >= 2962 && raw_char <= 2965)
        || (raw_char >= 2969 && raw_char <= 2970)
        || raw_char == 2972
        || (raw_char >= 2974 && raw_char <= 2975)
        || (raw_char >= 2979 && raw_char <= 2980)
        || (raw_char >= 2984 && raw_char <= 2986)
        || (raw_char >= 2990 && raw_char <= 3001)
        || raw_char == 3024
        || (raw_char >= 3077 && raw_char <= 3084)
        || (raw_char >= 3086 && raw_char <= 3088)
        || (raw_char >= 3090 && raw_char <= 3112)
        || (raw_char >= 3114 && raw_char <= 3129)
        || raw_char == 3133
        || (raw_char >= 3160 && raw_char <= 3162)
        || (raw_char >= 3168 && raw_char <= 3169)
        || raw_char == 3200
        || (raw_char >= 3205 && raw_char <= 3212)
        || (raw_char >= 3214 && raw_char <= 3216)
        || (raw_char >= 3218 && raw_char <= 3240)
        || (raw_char >= 3242 && raw_char <= 3251)
        || (raw_char >= 3253 && raw_char <= 3257)
        || raw_char == 3261
        || raw_char == 3294
        || (raw_char >= 3296 && raw_char <= 3297)
        || (raw_char >= 3313 && raw_char <= 3314)
        || (raw_char >= 3333 && raw_char <= 3340)
        || (raw_char >= 3342 && raw_char <= 3344)
        || (raw_char >= 3346 && raw_char <= 3386)
        || raw_char == 3389
        || raw_char == 3406
        || (raw_char >= 3412 && raw_char <= 3414)
        || (raw_char >= 3423 && raw_char <= 3425)
        || (raw_char >= 3450 && raw_char <= 3455)
        || (raw_char >= 3461 && raw_char <= 3478)
        || (raw_char >= 3482 && raw_char <= 3505)
        || (raw_char >= 3507 && raw_char <= 3515)
        || raw_char == 3517
        || (raw_char >= 3520 && raw_char <= 3526)
        || (raw_char >= 3585 && raw_char <= 3632)
        || (raw_char >= 3634 && raw_char <= 3635)
        || (raw_char >= 3648 && raw_char <= 3653)
        || (raw_char >= 3713 && raw_char <= 3714)
        || raw_char == 3716
        || (raw_char >= 3719 && raw_char <= 3720)
        || raw_char == 3722
        || raw_char == 3725
        || (raw_char >= 3732 && raw_char <= 3735)
        || (raw_char >= 3737 && raw_char <= 3743)
        || (raw_char >= 3745 && raw_char <= 3747)
        || raw_char == 3749
        || raw_char == 3751
        || (raw_char >= 3754 && raw_char <= 3755)
        || (raw_char >= 3757 && raw_char <= 3760)
        || (raw_char >= 3762 && raw_char <= 3763)
        || raw_char == 3773
        || (raw_char >= 3776 && raw_char <= 3780)
        || (raw_char >= 3804 && raw_char <= 3807)
        || raw_char == 3840
        || (raw_char >= 3904 && raw_char <= 3911)
        || (raw_char >= 3913 && raw_char <= 3948)
        || (raw_char >= 3976 && raw_char <= 3980)
        || (raw_char >= 4096 && raw_char <= 4138)
        || raw_char == 4159
        || (raw_char >= 4176 && raw_char <= 4181)
        || (raw_char >= 4186 && raw_char <= 4189)
        || raw_char == 4193
        || (raw_char >= 4197 && raw_char <= 4198)
        || (raw_char >= 4206 && raw_char <= 4208)
        || (raw_char >= 4213 && raw_char <= 4225)
        || raw_char == 4238
        || (raw_char >= 4352 && raw_char <= 4680)
        || (raw_char >= 4682 && raw_char <= 4685)
        || (raw_char >= 4688 && raw_char <= 4694)
        || raw_char == 4696
        || (raw_char >= 4698 && raw_char <= 4701)
        || (raw_char >= 4704 && raw_char <= 4744)
        || (raw_char >= 4746 && raw_char <= 4749)
        || (raw_char >= 4752 && raw_char <= 4784)
        || (raw_char >= 4786 && raw_char <= 4789)
        || (raw_char >= 4792 && raw_char <= 4798)
        || raw_char == 4800
        || (raw_char >= 4802 && raw_char <= 4805)
        || (raw_char >= 4808 && raw_char <= 4822)
        || (raw_char >= 4824 && raw_char <= 4880)
        || (raw_char >= 4882 && raw_char <= 4885)
        || (raw_char >= 4888 && raw_char <= 4954)
        || (raw_char >= 4992 && raw_char <= 5007)
        || (raw_char >= 5121 && raw_char <= 5740)
        || (raw_char >= 5743 && raw_char <= 5759)
        || (raw_char >= 5761 && raw_char <= 5786)
        || (raw_char >= 5792 && raw_char <= 5866)
        || (raw_char >= 5873 && raw_char <= 5880)
        || (raw_char >= 5888 && raw_char <= 5900)
        || (raw_char >= 5902 && raw_char <= 5905)
        || (raw_char >= 5920 && raw_char <= 5937)
        || (raw_char >= 5952 && raw_char <= 5969)
        || (raw_char >= 5984 && raw_char <= 5996)
        || (raw_char >= 5998 && raw_char <= 6000)
        || (raw_char >= 6016 && raw_char <= 6067)
        || raw_char == 6108
        || (raw_char >= 6176 && raw_char <= 6210)
        || (raw_char >= 6212 && raw_char <= 6264)
        || (raw_char >= 6272 && raw_char <= 6276)
        || (raw_char >= 6279 && raw_char <= 6312)
        || raw_char == 6314
        || (raw_char >= 6320 && raw_char <= 6389)
        || (raw_char >= 6400 && raw_char <= 6430)
        || (raw_char >= 6480 && raw_char <= 6509)
        || (raw_char >= 6512 && raw_char <= 6516)
        || (raw_char >= 6528 && raw_char <= 6571)
        || (raw_char >= 6576 && raw_char <= 6601)
        || (raw_char >= 6656 && raw_char <= 6678)
        || (raw_char >= 6688 && raw_char <= 6740)
        || (raw_char >= 6917 && raw_char <= 6963)
        || (raw_char >= 6981 && raw_char <= 6987)
        || (raw_char >= 7043 && raw_char <= 7072)
        || (raw_char >= 7086 && raw_char <= 7087)
        || (raw_char >= 7098 && raw_char <= 7141)
        || (raw_char >= 7168 && raw_char <= 7203)
        || (raw_char >= 7245 && raw_char <= 7247)
        || (raw_char >= 7258 && raw_char <= 7287)
        || (raw_char >= 7401 && raw_char <= 7404)
        || (raw_char >= 7406 && raw_char <= 7409)
        || (raw_char >= 7413 && raw_char <= 7414)
        || (raw_char >= 8501 && raw_char <= 8504)
        || (raw_char >= 11568 && raw_char <= 11623)
        || (raw_char >= 11648 && raw_char <= 11670)
        || (raw_char >= 11680 && raw_char <= 11686)
        || (raw_char >= 11688 && raw_char <= 11694)
        || (raw_char >= 11696 && raw_char <= 11702)
        || (raw_char >= 11704 && raw_char <= 11710)
        || (raw_char >= 11712 && raw_char <= 11718)
        || (raw_char >= 11720 && raw_char <= 11726)
        || (raw_char >= 11728 && raw_char <= 11734)
        || (raw_char >= 11736 && raw_char <= 11742)
        || raw_char == 12294
        || raw_char == 12348
        || (raw_char >= 12353 && raw_char <= 12438)
        || raw_char == 12447
        || (raw_char >= 12449 && raw_char <= 12538)
        || raw_char == 12543
        || (raw_char >= 12549 && raw_char <= 12591)
        || (raw_char >= 12593 && raw_char <= 12686)
        || (raw_char >= 12704 && raw_char <= 12730)
        || (raw_char >= 12784 && raw_char <= 12799)
        || raw_char == 13312
        || raw_char == 19893
        || raw_char == 19968
        || raw_char == 40943
        || (raw_char >= 40960 && raw_char <= 40980)
        || (raw_char >= 40982 && raw_char <= 42124)
        || (raw_char >= 42192 && raw_char <= 42231)
        || (raw_char >= 42240 && raw_char <= 42507)
        || (raw_char >= 42512 && raw_char <= 42527)
        || (raw_char >= 42538 && raw_char <= 42539)
        || raw_char == 42606
        || (raw_char >= 42656 && raw_char <= 42725)
        || raw_char == 42895
        || raw_char == 42999
        || (raw_char >= 43003 && raw_char <= 43009)
        || (raw_char >= 43011 && raw_char <= 43013)
        || (raw_char >= 43015 && raw_char <= 43018)
        || (raw_char >= 43020 && raw_char <= 43042)
        || (raw_char >= 43072 && raw_char <= 43123)
        || (raw_char >= 43138 && raw_char <= 43187)
        || (raw_char >= 43250 && raw_char <= 43255)
        || raw_char == 43259
        || (raw_char >= 43261 && raw_char <= 43262)
        || (raw_char >= 43274 && raw_char <= 43301)
        || (raw_char >= 43312 && raw_char <= 43334)
        || (raw_char >= 43360 && raw_char <= 43388)
        || (raw_char >= 43396 && raw_char <= 43442)
        || (raw_char >= 43488 && raw_char <= 43492)
        || (raw_char >= 43495 && raw_char <= 43503)
        || (raw_char >= 43514 && raw_char <= 43518)
        || (raw_char >= 43520 && raw_char <= 43560)
        || (raw_char >= 43584 && raw_char <= 43586)
        || (raw_char >= 43588 && raw_char <= 43595)
        || (raw_char >= 43616 && raw_char <= 43631)
        || (raw_char >= 43633 && raw_char <= 43638)
        || raw_char == 43642
        || (raw_char >= 43646 && raw_char <= 43695)
        || raw_char == 43697
        || (raw_char >= 43701 && raw_char <= 43702)
        || (raw_char >= 43705 && raw_char <= 43709)
        || raw_char == 43712
        || raw_char == 43714
        || (raw_char >= 43739 && raw_char <= 43740)
        || (raw_char >= 43744 && raw_char <= 43754)
        || raw_char == 43762
        || (raw_char >= 43777 && raw_char <= 43782)
        || (raw_char >= 43785 && raw_char <= 43790)
        || (raw_char >= 43793 && raw_char <= 43798)
        || (raw_char >= 43808 && raw_char <= 43814)
        || (raw_char >= 43816 && raw_char <= 43822)
        || (raw_char >= 43968 && raw_char <= 44002)
        || raw_char == 44032
        || raw_char == 55203
        || (raw_char >= 55216 && raw_char <= 55238)
        || (raw_char >= 55243 && raw_char <= 55291)
        || (raw_char >= 63744 && raw_char <= 64109)
        || (raw_char >= 64112 && raw_char <= 64217)
        || raw_char == 64285
        || (raw_char >= 64287 && raw_char <= 64296)
        || (raw_char >= 64298 && raw_char <= 64310)
        || (raw_char >= 64312 && raw_char <= 64316)
        || raw_char == 64318
        || (raw_char >= 64320 && raw_char <= 64321)
        || (raw_char >= 64323 && raw_char <= 64324)
        || (raw_char >= 64326 && raw_char <= 64433)
        || (raw_char >= 64467 && raw_char <= 64829)
        || (raw_char >= 64848 && raw_char <= 64911)
        || (raw_char >= 64914 && raw_char <= 64967)
        || (raw_char >= 65008 && raw_char <= 65019)
        || (raw_char >= 65136 && raw_char <= 65140)
        || (raw_char >= 65142 && raw_char <= 65276)
        || (raw_char >= 65382 && raw_char <= 65391)
        || (raw_char >= 65393 && raw_char <= 65437)
        || (raw_char >= 65440 && raw_char <= 65470)
        || (raw_char >= 65474 && raw_char <= 65479)
        || (raw_char >= 65482 && raw_char <= 65487)
        || (raw_char >= 65490 && raw_char <= 65495)
        || (raw_char >= 65498 && raw_char <= 65500)
        || (raw_char >= 65536 && raw_char <= 65547)
        || (raw_char >= 65549 && raw_char <= 65574)
        || (raw_char >= 65576 && raw_char <= 65594)
        || (raw_char >= 65596 && raw_char <= 65597)
        || (raw_char >= 65599 && raw_char <= 65613)
        || (raw_char >= 65616 && raw_char <= 65629)
        || (raw_char >= 65664 && raw_char <= 65786)
        || (raw_char >= 66176 && raw_char <= 66204)
        || (raw_char >= 66208 && raw_char <= 66256)
        || (raw_char >= 66304 && raw_char <= 66335)
        || (raw_char >= 66349 && raw_char <= 66368)
        || (raw_char >= 66370 && raw_char <= 66377)
        || (raw_char >= 66384 && raw_char <= 66421)
        || (raw_char >= 66432 && raw_char <= 66461)
        || (raw_char >= 66464 && raw_char <= 66499)
        || (raw_char >= 66504 && raw_char <= 66511)
        || (raw_char >= 66640 && raw_char <= 66717)
        || (raw_char >= 66816 && raw_char <= 66855)
        || (raw_char >= 66864 && raw_char <= 66915)
        || (raw_char >= 67072 && raw_char <= 67382)
        || (raw_char >= 67392 && raw_char <= 67413)
        || (raw_char >= 67424 && raw_char <= 67431)
        || (raw_char >= 67584 && raw_char <= 67589)
        || raw_char == 67592
        || (raw_char >= 67594 && raw_char <= 67637)
        || (raw_char >= 67639 && raw_char <= 67640)
        || raw_char == 67644
        || (raw_char >= 67647 && raw_char <= 67669)
        || (raw_char >= 67680 && raw_char <= 67702)
        || (raw_char >= 67712 && raw_char <= 67742)
        || (raw_char >= 67808 && raw_char <= 67826)
        || (raw_char >= 67828 && raw_char <= 67829)
        || (raw_char >= 67840 && raw_char <= 67861)
        || (raw_char >= 67872 && raw_char <= 67897)
        || (raw_char >= 67968 && raw_char <= 68023)
        || (raw_char >= 68030 && raw_char <= 68031)
        || raw_char == 68096
        || (raw_char >= 68112 && raw_char <= 68115)
        || (raw_char >= 68117 && raw_char <= 68119)
        || (raw_char >= 68121 && raw_char <= 68149)
        || (raw_char >= 68192 && raw_char <= 68220)
        || (raw_char >= 68224 && raw_char <= 68252)
        || (raw_char >= 68288 && raw_char <= 68295)
        || (raw_char >= 68297 && raw_char <= 68324)
        || (raw_char >= 68352 && raw_char <= 68405)
        || (raw_char >= 68416 && raw_char <= 68437)
        || (raw_char >= 68448 && raw_char <= 68466)
        || (raw_char >= 68480 && raw_char <= 68497)
        || (raw_char >= 68608 && raw_char <= 68680)
        || (raw_char >= 68864 && raw_char <= 68899)
        || (raw_char >= 69376 && raw_char <= 69404)
        || raw_char == 69415
        || (raw_char >= 69424 && raw_char <= 69445)
        || (raw_char >= 69635 && raw_char <= 69687)
        || (raw_char >= 69763 && raw_char <= 69807)
        || (raw_char >= 69840 && raw_char <= 69864)
        || (raw_char >= 69891 && raw_char <= 69926)
        || raw_char == 69956
        || (raw_char >= 69968 && raw_char <= 70002)
        || raw_char == 70006
        || (raw_char >= 70019 && raw_char <= 70066)
        || (raw_char >= 70081 && raw_char <= 70084)
        || raw_char == 70106
        || raw_char == 70108
        || (raw_char >= 70144 && raw_char <= 70161)
        || (raw_char >= 70163 && raw_char <= 70187)
        || (raw_char >= 70272 && raw_char <= 70278)
        || raw_char == 70280
        || (raw_char >= 70282 && raw_char <= 70285)
        || (raw_char >= 70287 && raw_char <= 70301)
        || (raw_char >= 70303 && raw_char <= 70312)
        || (raw_char >= 70320 && raw_char <= 70366)
        || (raw_char >= 70405 && raw_char <= 70412)
        || (raw_char >= 70415 && raw_char <= 70416)
        || (raw_char >= 70419 && raw_char <= 70440)
        || (raw_char >= 70442 && raw_char <= 70448)
        || (raw_char >= 70450 && raw_char <= 70451)
        || (raw_char >= 70453 && raw_char <= 70457)
        || raw_char == 70461
        || raw_char == 70480
        || (raw_char >= 70493 && raw_char <= 70497)
        || (raw_char >= 70656 && raw_char <= 70708)
        || (raw_char >= 70727 && raw_char <= 70730)
        || (raw_char >= 70784 && raw_char <= 70831)
        || (raw_char >= 70852 && raw_char <= 70853)
        || raw_char == 70855
        || (raw_char >= 71040 && raw_char <= 71086)
        || (raw_char >= 71128 && raw_char <= 71131)
        || (raw_char >= 71168 && raw_char <= 71215)
        || raw_char == 71236
        || (raw_char >= 71296 && raw_char <= 71338)
        || (raw_char >= 71424 && raw_char <= 71450)
        || (raw_char >= 71680 && raw_char <= 71723)
        || raw_char == 71935
        || raw_char == 72192
        || (raw_char >= 72203 && raw_char <= 72242)
        || raw_char == 72250
        || raw_char == 72272
        || (raw_char >= 72284 && raw_char <= 72323)
        || (raw_char >= 72326 && raw_char <= 72329)
        || raw_char == 72349
        || (raw_char >= 72384 && raw_char <= 72440)
        || (raw_char >= 72704 && raw_char <= 72712)
        || (raw_char >= 72714 && raw_char <= 72750)
        || raw_char == 72768
        || (raw_char >= 72818 && raw_char <= 72847)
        || (raw_char >= 72960 && raw_char <= 72966)
        || (raw_char >= 72968 && raw_char <= 72969)
        || (raw_char >= 72971 && raw_char <= 73008)
        || raw_char == 73030
        || (raw_char >= 73056 && raw_char <= 73061)
        || (raw_char >= 73063 && raw_char <= 73064)
        || (raw_char >= 73066 && raw_char <= 73097)
        || raw_char == 73112
        || (raw_char >= 73440 && raw_char <= 73458)
        || (raw_char >= 73728 && raw_char <= 74649)
        || (raw_char >= 74880 && raw_char <= 75075)
        || (raw_char >= 77824 && raw_char <= 78894)
        || (raw_char >= 82944 && raw_char <= 83526)
    })
}

pub fn zs<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        raw_char == 32
        || raw_char == 160
        || raw_char == 5760
        || (raw_char >= 8192 && raw_char <= 8202)
        || raw_char == 8239
        || raw_char == 8287
    })
}

pub fn pc<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        raw_char == 95
        || (raw_char >= 8255 && raw_char <= 8256)
        || raw_char == 8276
        || (raw_char >= 65075 && raw_char <= 65076)
        || (raw_char >= 65101 && raw_char <= 65103)
    })
}

pub fn nd<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        (raw_char >= 48 && raw_char <= 57)
        || (raw_char >= 1632 && raw_char <= 1641)
        || (raw_char >= 1776 && raw_char <= 1785)
        || (raw_char >= 1984 && raw_char <= 1993)
        || (raw_char >= 2406 && raw_char <= 2415)
        || (raw_char >= 2534 && raw_char <= 2543)
        || (raw_char >= 2662 && raw_char <= 2671)
        || (raw_char >= 2790 && raw_char <= 2799)
        || (raw_char >= 2918 && raw_char <= 2927)
        || (raw_char >= 3046 && raw_char <= 3055)
        || (raw_char >= 3174 && raw_char <= 3183)
        || (raw_char >= 3302 && raw_char <= 3311)
        || (raw_char >= 3430 && raw_char <= 3439)
        || (raw_char >= 3558 && raw_char <= 3567)
        || (raw_char >= 3664 && raw_char <= 3673)
        || (raw_char >= 3792 && raw_char <= 3801)
        || (raw_char >= 3872 && raw_char <= 3881)
        || (raw_char >= 4160 && raw_char <= 4169)
        || (raw_char >= 4240 && raw_char <= 4249)
        || (raw_char >= 6112 && raw_char <= 6121)
        || (raw_char >= 6160 && raw_char <= 6169)
        || (raw_char >= 6470 && raw_char <= 6479)
        || (raw_char >= 6608 && raw_char <= 6617)
        || (raw_char >= 6784 && raw_char <= 6793)
        || (raw_char >= 6800 && raw_char <= 6809)
        || (raw_char >= 6992 && raw_char <= 7001)
        || (raw_char >= 7088 && raw_char <= 7097)
        || (raw_char >= 7232 && raw_char <= 7241)
        || (raw_char >= 7248 && raw_char <= 7257)
        || (raw_char >= 42528 && raw_char <= 42537)
        || (raw_char >= 43216 && raw_char <= 43225)
        || (raw_char >= 43264 && raw_char <= 43273)
        || (raw_char >= 43472 && raw_char <= 43481)
        || (raw_char >= 43504 && raw_char <= 43513)
        || (raw_char >= 43600 && raw_char <= 43609)
        || (raw_char >= 44016 && raw_char <= 44025)
        || (raw_char >= 65296 && raw_char <= 65305)
        || (raw_char >= 66720 && raw_char <= 66729)
        || (raw_char >= 68912 && raw_char <= 68921)
        || (raw_char >= 69734 && raw_char <= 69743)
        || (raw_char >= 69872 && raw_char <= 69881)
        || (raw_char >= 69942 && raw_char <= 69951)
        || (raw_char >= 70096 && raw_char <= 70105)
        || (raw_char >= 70384 && raw_char <= 70393)
        || (raw_char >= 70736 && raw_char <= 70745)
        || (raw_char >= 70864 && raw_char <= 70873)
        || (raw_char >= 71248 && raw_char <= 71257)
        || (raw_char >= 71360 && raw_char <= 71369)
        || (raw_char >= 71472 && raw_char <= 71481)
        || (raw_char >= 71904 && raw_char <= 71913)
        || (raw_char >= 72784 && raw_char <= 72793)
        || (raw_char >= 73040 && raw_char <= 73049)
        || (raw_char >= 73120 && raw_char <= 73129)
        || (raw_char >= 92768 && raw_char <= 92777)
        || (raw_char >= 93008 && raw_char <= 93017)
        || (raw_char >= 120782 && raw_char <= 120831)
    })
}
pub fn nl<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        let raw_char = c as u32;
        (raw_char >= 5870 && raw_char <= 5872)
        || (raw_char >= 8544 && raw_char <= 8578)
        || (raw_char >= 8581 && raw_char <= 8584)
        || raw_char == 12295
        || (raw_char >= 12321 && raw_char <= 12329)
        || (raw_char >= 12344 && raw_char <= 12346)
        || (raw_char >= 42726 && raw_char <= 42735)
        || (raw_char >= 65856 && raw_char <= 65908)
        || raw_char == 66369
        || raw_char == 66378
        || (raw_char >= 66513 && raw_char <= 66517)
    })
}

pub fn escape_sequence<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('u')
        .and(
            count(4,
                hex_digit())
            )
    .map(|(u, hex): (char, String)| format!("{}{}", u, hex))
}

pub fn char_literal<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('\\').and(escape_sequence()).map(|(_, sequence):(char, String)| {
        format!("\\u{{{0}}}", &sequence[1..]).parse().unwrap()
    })
}