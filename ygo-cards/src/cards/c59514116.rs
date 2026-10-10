//! Secrets of Dark Magic
use super::prelude::*;

pub const ID: u32 = 59514116;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion
    // local e1=Fusion.CreateSummonEff({handler=c,desc=aux.Stringid(id,0),extrafil=function()return nil,s.matcheck end})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            desc: Some(aux::stringid(ID, 0)),
            extrafil: Some(std::sync::Arc::new(|_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet| (None, Some(lua_fn(matcheck))))),
            ..Default::default()
        },
    );
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_cost(d, e1, Some(cost_target(costhint)));
    card::register_effect(d, c, e1, false);
    // Ritual
    // local e2=Ritual.CreateProc({handler=c,lvtype=RITPROC_GREATER,desc=aux.Stringid(id,1),forcedselection=s.forcedselection})
    let e2 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            desc: Some(aux::stringid(ID, 1)),
            forcedselection: Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, tp: u8, sg: &CardSet, sc: CardId| (forcedselection(d, e, tp, sg, sc), false))),
            ..Default::default()
        },
    );
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_cost(d, e2, Some(cost_target(costhint)));
    card::register_effect(d, c, e2, false);
}
/// `s.costhint(e,tp,eg,ep,ev,re,r,rp,chk)`
fn costhint(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, effect::get_description(d, a.e));
    true
}
/// `s.matcheck(tp,sg,fc)`
fn matcheck(d: &mut Duel, a: Args) -> bool {
    let (tp, sg, fc) = (a.p(0), a.g(1), a.c(2));
    group::is_exists(d, &sg, &|d, c| card::is_summon_code(d, c, Some(fc), SUMMON_TYPE_FUSION as u64, tp, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]), 1, NO_EX)
}
/// `s.forcedselection(e,tp,sg,sc)`
fn forcedselection(d: &mut Duel, _e: EffectId, _tp: u8, sg: &CardSet, _sc: CardId) -> bool {
    group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]), 1, NO_EX)
}
