//! Garura, Wings of Resonant Life
use super::prelude::*;

pub const ID: u32 = 11765832;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion materials: 2 monsters with the same Type and Attribute, but different names
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // Any battle damage your opponent takes from battles involving this card is doubled
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ffilter(c,fc,sumtype,sump,sub,matg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    let (fc, sumtype, sump) = (a.fc, a.sumtype, a.tp);
    // return not sg or sg:FilterCount(aux.TRUE,c)==0 or (...)
    let Some(sg) = a.sg else { return true };
    if group::filter_count(d, sg, &|_d, _c| true, c.into()) == 0 {
        return true;
    }
    // sg:IsExists(Card.IsAttribute,1,c,c:GetAttribute(fc,sumtype,sump),fc,sumtype,sump)
    let att = card::get_attribute_ctx(d, c, Some(fc), sumtype, Some(sump));
    group::is_exists(d, sg, &|d, x| card::is_attribute_ctx(d, x, att, Some(fc), sumtype, Some(sump)), 1, c.into())
        && {
            // sg:IsExists(Card.IsRace,1,c,c:GetRace(fc,sumtype,sump),fc,sumtype,sump)
            let race = card::get_race_ctx(d, c, Some(fc), sumtype, Some(sump));
            group::is_exists(d, sg, &|d, x| card::is_race_ctx(d, x, race, Some(fc), sumtype, Some(sump)), 1, c.into())
        }
        && {
            // not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,sump),fc,sumtype,sump)
            let codes = card::get_code_ctx(d, c, Some(fc), sumtype, Some(sump));
            let code = codes.first().copied().nil();
            !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, sumtype, sump), 1, c.into())
        }
}
/// `s.fusfilter(c,code,fc,sumtype,sump)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, sumtype: u64, sump: u8) -> bool {
    // return c:IsSummonCode(fc,sumtype,sump,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), sumtype, sump, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dn) = (info[0].int() as u8, info[1].int() as u32);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, dn, REASON_EFFECT);
}
