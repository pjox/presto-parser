use std::{
    collections::{HashSet},
};

use structopt::lazy_static::lazy_static;

lazy_static! {
    pub static ref POS: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("Ag");
        m.insert("Ap");
        m.insert("Cc");
        m.insert("Cs");
        m.insert("Da");
        m.insert("Dd");
        m.insert("Di");
        m.insert("Dn");
        m.insert("Dp");
        m.insert("Ds");
        m.insert("Dt");
        m.insert("Fo");
        m.insert("Fp");
        m.insert("Fs");
        m.insert("Fw");
        m.insert("Ga");
        m.insert("Ge");
        m.insert("I");
        m.insert("Mc");
        m.insert("Mo");
        m.insert("Mp");
        m.insert("Nc");
        m.insert("Np");
        m.insert("Ns");
        m.insert("Nv");
        m.insert("Pd");
        m.insert("Pi");
        m.insert("Pp");
        m.insert("Pr");
        m.insert("Ps");
        m.insert("Pt");
        m.insert("Rg");
        m.insert("Rn");
        m.insert("Rp");
        m.insert("S");
        m.insert("S+Da");
        m.insert("S+Dr");
        m.insert("S+Pd");
        m.insert("S+Pr");
        m.insert("S+Rg");
        m.insert("S+Sa");
        m.insert("Vc");
        m.insert("Vcc");
        m.insert("Vuc");
        m.insert("Vun");
        m.insert("Vvc");
        m.insert("Vvn");
        m.insert("Vvx");
        m.insert("XXX");
        m.insert("Xa");
        m.insert("Xe");
        m.insert("Xi");
        m.insert("Xs");

        m
    };

    pub static ref COARSE: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("B-amount");
        m.insert("B-event");
        m.insert("B-func");
        m.insert("B-loc");
        m.insert("B-org");
        m.insert("B-pers");
        m.insert("B-prod");
        m.insert("B-time");
        m.insert("I-amount");
        m.insert("I-event");
        m.insert("I-func");
        m.insert("I-loc");
        m.insert("I-org");
        m.insert("I-pers");
        m.insert("I-prod");
        m.insert("I-time");
        m.insert("O");

        m
    };

    pub static ref FINE: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("B-amount");
        m.insert("B-event");
        m.insert("B-func");
        m.insert("B-func.coll");
        m.insert("B-func.ind");
        m.insert("B-loc");
        m.insert("B-loc.adm");
        m.insert("B-loc.adm.nat");
        m.insert("B-loc.adm.reg");
        m.insert("B-loc.adm.sup");
        m.insert("B-loc.adm.town");
        m.insert("B-loc.fac");
        m.insert("B-loc.oro");
        m.insert("B-loc.phys.geo");
        m.insert("B-loc.phys.hydro");
        m.insert("B-loc.unk");
        m.insert("B-org.adm");
        m.insert("B-org.ent");
        m.insert("B-per.ind");
        m.insert("B-pers");
        m.insert("B-pers.coll");
        m.insert("B-pers.ind");
        m.insert("B-prod.art");
        m.insert("B-prod.media");
        m.insert("B-prod.object");
        m.insert("B-prod.rule");
        m.insert("B-time.date.abs");
        m.insert("B-time.date.rel");
        m.insert("I-amount");
        m.insert("I-event");
        m.insert("I-func");
        m.insert("I-func.coll");
        m.insert("I-func.ind");
        m.insert("I-loc");
        m.insert("I-loc.adm.nat");
        m.insert("I-loc.adm.reg");
        m.insert("I-loc.adm.sup");
        m.insert("I-loc.adm.town");
        m.insert("I-loc.fac");
        m.insert("I-loc.oro");
        m.insert("I-loc.phys.geo");
        m.insert("I-loc.phys.hydro");
        m.insert("I-loc.unk");
        m.insert("I-org.adm");
        m.insert("I-org.ent");
        m.insert("I-pers");
        m.insert("I-pers.coll");
        m.insert("I-pers.ind");
        m.insert("I-prod.art");
        m.insert("I-prod.media");
        m.insert("I-prod.rule");
        m.insert("I-time.date.abs");
        m.insert("I-time.date.rel");
        m.insert("O");

        m
    };

    pub static ref COMP: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("B-comp.func");
        m.insert("B-comp.kind");
        m.insert("B-comp.name");
        m.insert("B-comp.qualifier");
        m.insert("B-comp.range-mark");
        m.insert("B-comp.time");
        m.insert("B-comp.title");
        m.insert("B-comp.unit");
        m.insert("B-comp.val");
        m.insert("I-comp.func");
        m.insert("I-comp.kind");
        m.insert("I-comp.name");
        m.insert("I-comp.qualifier");
        m.insert("I-comp.title");
        m.insert("I-comp.unit");
        m.insert("I-comp.val");
        m.insert("O");

        m
    };

    pub static ref NESTED: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("B-amount");
        m.insert("B-event");
        m.insert("B-func");
        m.insert("B-func.coll");
        m.insert("B-func.ind");
        m.insert("B-loc");
        m.insert("B-loc.adm");
        m.insert("B-loc.adm.nat");
        m.insert("B-loc.adm.reg");
        m.insert("B-loc.adm.sup");
        m.insert("B-loc.adm.town");
        m.insert("B-loc.fac");
        m.insert("B-loc.oro");
        m.insert("B-loc.phys.geo");
        m.insert("B-loc.phys.hydro");
        m.insert("B-loc.unk");
        m.insert("B-org.adm");
        m.insert("B-org.ent");
        m.insert("B-per.ind");
        m.insert("B-pers");
        m.insert("B-pers.coll");
        m.insert("B-pers.ind");
        m.insert("B-prod.art");
        m.insert("B-prod.media");
        m.insert("B-prod.object");
        m.insert("B-prod.rule");
        m.insert("B-time.date.abs");
        m.insert("B-time.date.rel");
        m.insert("I-amount");
        m.insert("I-event");
        m.insert("I-func");
        m.insert("I-func.coll");
        m.insert("I-func.ind");
        m.insert("I-loc");
        m.insert("I-loc.adm.nat");
        m.insert("I-loc.adm.reg");
        m.insert("I-loc.adm.sup");
        m.insert("I-loc.adm.town");
        m.insert("I-loc.fac");
        m.insert("I-loc.oro");
        m.insert("I-loc.phys.geo");
        m.insert("I-loc.phys.hydro");
        m.insert("I-loc.unk");
        m.insert("I-org.adm");
        m.insert("I-org.ent");
        m.insert("I-pers");
        m.insert("I-pers.coll");
        m.insert("I-pers.ind");
        m.insert("I-prod.art");
        m.insert("I-prod.media");
        m.insert("I-prod.rule");
        m.insert("I-time.date.abs");
        m.insert("I-time.date.rel");
        m.insert("O");

        m
    };
}
