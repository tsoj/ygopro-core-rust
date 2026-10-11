//! Gouki Heel Ogre
use super::prelude::*;

pub const ID: u32 = 10552026;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_GOUKI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GOUKI as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // Negate monster effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `bit.extract(v,pos)`
fn bit_extract(v: u32, pos: i64) -> u32 {
    if !(0..64).contains(&pos) {
        return 0;
    }
    (((v as u64) >> pos) & 1) as u32
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // local loc,seq,p=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_SEQUENCE,CHAININFO_TRIGGERING_CONTROLER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE, CHAININFO_TRIGGERING_CONTROLER]);
    let loc = info.first().map_or(0, |v| v.int());
    let mut seq = info.get(1).map_or(0, |v| v.int());
    let p = info.get(2).map_or(0, |v| v.int());
    // if p==1-tp then seq=seq+16 end
    if p == (1 - a.tp) as i64 {
        seq += 16;
    }
    // return re:IsMonsterEffect() and (loc&LOCATION_MZONE)~=0 and bit.extract(c:GetLinkedZone(),seq)~=0 and Duel.IsChainNegatable(ev)
    effect::is_monster_effect(d, a.re.nil())
        && (loc & LOCATION_MZONE as i64) != 0
        && bit_extract(card::get_linked_zone(d, c, None), seq) != 0
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOUKI) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if !card::is_relate_to_effect(d, rh, a.re.nil()) {
        return;
    }
    if duel::destroy(d, a.eg(), REASON_EFFECT, None, None) <= 0 {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
    let nvf = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let g = duel::get_matching_group(d, Some(&nvf), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
