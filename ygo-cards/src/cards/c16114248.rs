//! Pair Cycroid
use super::prelude::*;

pub const ID: u32 = 16114248;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ffilter(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    let (fc, sumtype, tp) = (a.fc, a.sumtype, a.tp);
    // return c:IsRace(RACE_MACHINE,fc,sumtype,tp) and (not sg or sg:FilterCount(aux.TRUE,c)==0
    //   or sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,tp),fc,tp))
    if !card::is_race_ctx(d, c, RACE_MACHINE as u64, Some(fc), sumtype, Some(tp)) {
        return false;
    }
    let Some(sg) = a.sg else { return true };
    if group::filter_count(d, sg, &|_d, _c| true, c.into()) == 0 {
        return true;
    }
    let codes = card::get_code_ctx(d, c, Some(fc), sumtype, Some(tp));
    let code = codes.first().copied().nil();
    group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, tp), 1, c.into())
}
/// `s.fusfilter(c,code,fc,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, tp: u8) -> bool {
    // return c:IsSummonCode(fc,SUMMON_TYPE_FUSION,tp,code) or c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), SUMMON_TYPE_FUSION as u64, tp, &[code]) || !card::is_has_effect(d, c, 511002961, None).is_empty()
}
