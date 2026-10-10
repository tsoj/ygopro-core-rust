//! Prediction Princess Tarotreith
use super::prelude::*;

pub const ID: u32 = 42932862;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[8428836, 94997874], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.spconvalue)
    effect::set_value_fn(d, e0, lua_fn(spconvalue));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // FLIP: Special Summon 1 Flip monster from Deck.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Flip face-up or face-down any number of monsters.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_END_PHASE,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconvalue(e,se,sp,st)`
fn spconvalue(d: &mut Duel, a: Args) -> bool {
    // return aux.ritlimit(e,se,sp,st) or se:GetHandler():IsCode(94997874)
    if aux::ritlimit().call(d, a.0).truthy() {
        return true;
    }
    let se = a.e(1);
    let h = effect::get_handler(d, se).nil();
    card::is_code(d, h, &[94997874])
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FLIP) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_type(d, c, TYPE_FLIP) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.IsExistingMatchingCard(Card.IsFacedown,tp,LOCATION_MZONE,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.posfilter,tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    // (never nil here: a choice is possible when the check passed)
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // local category=op==1 and CATEGORY_POSITION or (CATEGORY_POSITION+CATEGORY_SET)
    let category = if op == 1 { CATEGORY_POSITION } else { CATEGORY_POSITION + CATEGORY_SET };
    // local pos=op==1 and POS_FACEUP_DEFENSE or POS_FACEDOWN_DEFENSE
    let pos = if op == 1 { POS_FACEUP_DEFENSE } else { POS_FACEDOWN_DEFENSE };
    // e:SetCategory(category)
    effect::set_category(d, e, category);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,pos)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, tp, pos as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local filter=op==1 and Card.IsFacedown or s.posfilter
    // local g=Duel.GetMatchingGroup(filter,tp,LOCATION_MZONE,0,nil)
    let g = if op == 1 {
        duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, NO_EX)
    } else {
        duel::get_matching_group(d, Some(&posfilter), tp, LOCATION_MZONE, 0, NO_EX)
    };
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local pos=op==1 and POS_FACEUP_DEFENSE or POS_FACEDOWN_DEFENSE
    let pos = if op == 1 { POS_FACEUP_DEFENSE } else { POS_FACEDOWN_DEFENSE };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // Duel.ChangePosition(g:Select(tp,1,#g,nil),pos)
    let max = g.len() as u16;
    let sel = group::select(d, &g, tp, 1, max, false, NO_EX).nil();
    duel::change_position(d, sel, pos as u8, None, None, None, false);
}
