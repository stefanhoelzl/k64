# k64
Peripheral access API for Kinetis K64 microcontrollers

# memory.x
A memory.x file can be found on the repository, but it only uses one section of the SRAM memory, feel free to change the script to support both sections.

# Issues
Due to [svd2rust Issue 16](https://github.com/japaric/svd2rust/issues/16) there are some registers missing.
```bash
WARNING Some(Ident("tcd0_nbytes_mloffno")) overlaps with another register block at offset 4104. Ignoring.
WARNING Some(Ident("tcd0_nbytes_mloffyes")) overlaps with another register block at offset 4104. Ignoring.
WARNING Some(Ident("tcd0_citer_elinkyes")) overlaps with another register block at offset 4118. Ignoring.
WARNING Some(Ident("tcd0_biter_elinkyes")) overlaps with another register block at offset 4126. Ignoring.
WARNING Some(Ident("tcd1_nbytes_mloffno")) overlaps with another register block at offset 4136. Ignoring.
WARNING Some(Ident("tcd1_nbytes_mloffyes")) overlaps with another register block at offset 4136. Ignoring.
WARNING Some(Ident("tcd1_citer_elinkyes")) overlaps with another register block at offset 4150. Ignoring.
WARNING Some(Ident("tcd1_biter_elinkyes")) overlaps with another register block at offset 4158. Ignoring.
WARNING Some(Ident("tcd2_nbytes_mloffno")) overlaps with another register block at offset 4168. Ignoring.
WARNING Some(Ident("tcd2_nbytes_mloffyes")) overlaps with another register block at offset 4168. Ignoring.
WARNING Some(Ident("tcd2_citer_elinkyes")) overlaps with another register block at offset 4182. Ignoring.
WARNING Some(Ident("tcd2_biter_elinkyes")) overlaps with another register block at offset 4190. Ignoring.
WARNING Some(Ident("tcd3_nbytes_mloffno")) overlaps with another register block at offset 4200. Ignoring.
WARNING Some(Ident("tcd3_nbytes_mloffyes")) overlaps with another register block at offset 4200. Ignoring.
WARNING Some(Ident("tcd3_citer_elinkyes")) overlaps with another register block at offset 4214. Ignoring.
WARNING Some(Ident("tcd3_biter_elinkyes")) overlaps with another register block at offset 4222. Ignoring.
WARNING Some(Ident("tcd4_nbytes_mloffno")) overlaps with another register block at offset 4232. Ignoring.
WARNING Some(Ident("tcd4_nbytes_mloffyes")) overlaps with another register block at offset 4232. Ignoring.
WARNING Some(Ident("tcd4_citer_elinkyes")) overlaps with another register block at offset 4246. Ignoring.
WARNING Some(Ident("tcd4_biter_elinkyes")) overlaps with another register block at offset 4254. Ignoring.
WARNING Some(Ident("tcd5_nbytes_mloffno")) overlaps with another register block at offset 4264. Ignoring.
WARNING Some(Ident("tcd5_nbytes_mloffyes")) overlaps with another register block at offset 4264. Ignoring.
WARNING Some(Ident("tcd5_citer_elinkyes")) overlaps with another register block at offset 4278. Ignoring.
WARNING Some(Ident("tcd5_biter_elinkyes")) overlaps with another register block at offset 4286. Ignoring.
WARNING Some(Ident("tcd6_nbytes_mloffno")) overlaps with another register block at offset 4296. Ignoring.
WARNING Some(Ident("tcd6_nbytes_mloffyes")) overlaps with another register block at offset 4296. Ignoring.
WARNING Some(Ident("tcd6_citer_elinkyes")) overlaps with another register block at offset 4310. Ignoring.
WARNING Some(Ident("tcd6_biter_elinkyes")) overlaps with another register block at offset 4318. Ignoring.
WARNING Some(Ident("tcd7_nbytes_mloffno")) overlaps with another register block at offset 4328. Ignoring.
WARNING Some(Ident("tcd7_nbytes_mloffyes")) overlaps with another register block at offset 4328. Ignoring.
WARNING Some(Ident("tcd7_citer_elinkyes")) overlaps with another register block at offset 4342. Ignoring.
WARNING Some(Ident("tcd7_biter_elinkyes")) overlaps with another register block at offset 4350. Ignoring.
WARNING Some(Ident("tcd8_nbytes_mloffno")) overlaps with another register block at offset 4360. Ignoring.
WARNING Some(Ident("tcd8_nbytes_mloffyes")) overlaps with another register block at offset 4360. Ignoring.
WARNING Some(Ident("tcd8_citer_elinkyes")) overlaps with another register block at offset 4374. Ignoring.
WARNING Some(Ident("tcd8_biter_elinkyes")) overlaps with another register block at offset 4382. Ignoring.
WARNING Some(Ident("tcd9_nbytes_mloffno")) overlaps with another register block at offset 4392. Ignoring.
WARNING Some(Ident("tcd9_nbytes_mloffyes")) overlaps with another register block at offset 4392. Ignoring.
WARNING Some(Ident("tcd9_citer_elinkyes")) overlaps with another register block at offset 4406. Ignoring.
WARNING Some(Ident("tcd9_biter_elinkyes")) overlaps with another register block at offset 4414. Ignoring.
WARNING Some(Ident("tcd10_nbytes_mloffno")) overlaps with another register block at offset 4424. Ignoring.
WARNING Some(Ident("tcd10_nbytes_mloffyes")) overlaps with another register block at offset 4424. Ignoring.
WARNING Some(Ident("tcd10_citer_elinkyes")) overlaps with another register block at offset 4438. Ignoring.
WARNING Some(Ident("tcd10_biter_elinkyes")) overlaps with another register block at offset 4446. Ignoring.
WARNING Some(Ident("tcd11_nbytes_mloffno")) overlaps with another register block at offset 4456. Ignoring.
WARNING Some(Ident("tcd11_nbytes_mloffyes")) overlaps with another register block at offset 4456. Ignoring.
WARNING Some(Ident("tcd11_citer_elinkyes")) overlaps with another register block at offset 4470. Ignoring.
WARNING Some(Ident("tcd11_biter_elinkyes")) overlaps with another register block at offset 4478. Ignoring.
WARNING Some(Ident("tcd12_nbytes_mloffno")) overlaps with another register block at offset 4488. Ignoring.
WARNING Some(Ident("tcd12_nbytes_mloffyes")) overlaps with another register block at offset 4488. Ignoring.
WARNING Some(Ident("tcd12_citer_elinkyes")) overlaps with another register block at offset 4502. Ignoring.
WARNING Some(Ident("tcd12_biter_elinkyes")) overlaps with another register block at offset 4510. Ignoring.
WARNING Some(Ident("tcd13_nbytes_mloffno")) overlaps with another register block at offset 4520. Ignoring.
WARNING Some(Ident("tcd13_nbytes_mloffyes")) overlaps with another register block at offset 4520. Ignoring.
WARNING Some(Ident("tcd13_citer_elinkyes")) overlaps with another register block at offset 4534. Ignoring.
WARNING Some(Ident("tcd13_biter_elinkyes")) overlaps with another register block at offset 4542. Ignoring.
WARNING Some(Ident("tcd14_nbytes_mloffno")) overlaps with another register block at offset 4552. Ignoring.
WARNING Some(Ident("tcd14_nbytes_mloffyes")) overlaps with another register block at offset 4552. Ignoring.
WARNING Some(Ident("tcd14_citer_elinkyes")) overlaps with another register block at offset 4566. Ignoring.
WARNING Some(Ident("tcd14_biter_elinkyes")) overlaps with another register block at offset 4574. Ignoring.
WARNING Some(Ident("tcd15_nbytes_mloffno")) overlaps with another register block at offset 4584. Ignoring.
WARNING Some(Ident("tcd15_nbytes_mloffyes")) overlaps with another register block at offset 4584. Ignoring.
WARNING Some(Ident("tcd15_citer_elinkyes")) overlaps with another register block at offset 4598. Ignoring.
WARNING Some(Ident("tcd15_biter_elinkyes")) overlaps with another register block at offset 4606. Ignoring.
WARNING Some(Ident("ctar_slave")) overlaps with another register block at offset 12. Ignoring.
WARNING Some(Ident("pushr_slave")) overlaps with another register block at offset 52. Ignoring.
WARNING Some(Ident("ctar_slave")) overlaps with another register block at offset 12. Ignoring.
WARNING Some(Ident("pushr_slave")) overlaps with another register block at offset 52. Ignoring.
WARNING Some(Ident("ctar_slave")) overlaps with another register block at offset 12. Ignoring.
WARNING Some(Ident("pushr_slave")) overlaps with another register block at offset 52. Ignoring.
WARNING Some(Ident("datal")) overlaps with another register block at offset 0. Ignoring.
WARNING Some(Ident("datall")) overlaps with another register block at offset 0. Ignoring.
WARNING Some(Ident("datalu")) overlaps with another register block at offset 1. Ignoring.
WARNING Some(Ident("datah")) overlaps with another register block at offset 2. Ignoring.
WARNING Some(Ident("datahl")) overlaps with another register block at offset 2. Ignoring.
WARNING Some(Ident("datahu")) overlaps with another register block at offset 3. Ignoring.
WARNING Some(Ident("gpolyl")) overlaps with another register block at offset 4. Ignoring.
WARNING Some(Ident("gpolyll")) overlaps with another register block at offset 4. Ignoring.
WARNING Some(Ident("gpolylu")) overlaps with another register block at offset 5. Ignoring.
WARNING Some(Ident("gpolyh")) overlaps with another register block at offset 6. Ignoring.
WARNING Some(Ident("gpolyhl")) overlaps with another register block at offset 6. Ignoring.
WARNING Some(Ident("gpolyhu")) overlaps with another register block at offset 7. Ignoring.
WARNING Some(Ident("ctrlhu")) overlaps with another register block at offset 11. Ignoring.
WARNING Some(Ident("timer2_bc12")) overlaps with another register block at offset 24. Ignoring.
WARNING Some(Ident("wp7816t1")) overlaps with another register block at offset 27. Ignoring.
```
