//! Heavy Polymerization
use super::prelude::*;

pub const ID: u32 = 58570206;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion.RegisterSummonEff{handler=c,mincount=3,extrafil=s.fextra,extratg=s.extratg,extraop=s.extraop,stage2=s.stage2}
    fusion::register_summon_eff(
        d,
        c,
        fusion::SummonEff {
            mincount: Some(3),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            extraop: Some(fusion::extra_op(extraop)),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)<=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX) as i32 <= duel::get_field_group_count(d, tp, 0, LOCATION_MZONE)
}
/// `s.fextra(e,tp,mg)`: (the extra materials, the check)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local eg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_EXTRA,0,nil)
    let eg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #eg>0 then return eg,s.fcheck end
    if !eg.is_empty() {
        return (Some(eg), Some(lua_fn(fcheck)));
    }
    // return nil
    (None, None)
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_EXTRA)
    let rg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
        duel::remove(d, &rg, POS_FACEUP as u8, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None, None);
        // e:SetLabel(rg:GetSum(Card.GetAttack))
        let sum = group::get_sum(d, &rg, &|d, c| card::get_attack(d, c) as i64);
        effect::set_label(d, e, &[sum]);
        // sg:Sub(rg)
        group::remove_card(sg, &rg);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
    }
    true
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // local ct=e:GetLabel()
        let ct = effect::get_label(d, e);
        // if ct>0 then Duel.SetLP(tp,Duel.GetLP(tp)-ct) end
        if ct > 0 {
            let lp = duel::get_lp(d, tp);
            duel::set_lp(d, tp, lp - ct as i32);
        }
    }
}
