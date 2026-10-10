//! Mekk-Knight Indigo Eclipse
use super::prelude::*;

pub const ID: u32 = 92204263;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEKK_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.hspval)
    effect::set_value_fn(d, e1, lua_fn(hspval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Move 1 "Mekk-Knight" to another MMZ
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.seqtg)
    effect::set_target(d, e2, Some(cost_target(seqtg)));
    // e2:SetOperation(s.seqop)
    effect::set_operation(d, e2, Some(operation(seqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetColumnGroupCount()>0
    card::get_column_group_count(d, c, 0, 0) > 0
}
/// `s.hspval(e,c)`
fn hspval(d: &mut Duel, a: Args) -> Value {
    // local tp=c:GetControler()
    let c = a.c(1);
    let tp = card::get_controler(d, c);
    // local zone=0
    let mut zone: u32 = 0;
    // local lg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let lg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // for tc in aux.Next(lg) do
    for tc in group::iter(d, &lg) {
        // zone=(zone|tc:GetColumnZone(LOCATION_MZONE,0,0,tp))
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, 0, 0, Some(tp));
    }
    // return 0,zone
    Value::Multi(vec![Value::Int(0), Value::Int(zone as i64)])
}
/// `s.seqfilter(c)`
fn seqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MEKK_KNIGHT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16])
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.seqfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && seqfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.seqfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&seqfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // Duel.SelectTarget(tp,s.seqfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&seqfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if !card::is_relate_to_effect(d, tc, e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, e)
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0),2))
    let zone = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(0), false);
    duel::move_sequence(d, tc, (zone as f64).log2() as u8, None);
}
