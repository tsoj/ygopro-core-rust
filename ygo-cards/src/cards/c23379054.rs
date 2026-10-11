//! Ghost Charon, the Underworld Boatman
use super::prelude::*;

pub const ID: u32 = 23379054;

/// The `mc` argument of `s.filter2`: a card (from `s.filter1`) or a group (from `s.operation`)
#[derive(Clone)]
enum Mc {
    Card(CardId),
    Group(CardSet),
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsRace(RACE_DRAGON)
    card::is_race(d, a.c(0), RACE_DRAGON)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==1
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0 && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 1
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetRace()~=RACE_DRAGON
    card::get_race(d, c) != RACE_DRAGON
}
/// `s.filter1(c,e,tp,lv,mc)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32, mc: CardId) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv+c:GetLevel(),mc)
    if !(card::is_type(d, c, TYPE_FUSION) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)) {
        return false;
    }
    let lv2 = lv + card::get_level(d, c);
    let mc = Mc::Card(mc);
    duel::is_existing_matching_card(d, Some(&|d, y| filter2(d, y, e, tp, lv2, &mc)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp,lv,mc)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32, mc: &Mc) -> bool {
    // return c:GetLevel()==lv and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    if !(card::get_level(d, c) == lv && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO)) {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let count = match mc {
        Mc::Card(x) => duel::get_location_count_from_ex(d, tp, Some(tp), (*x).into(), duel::ExtraCard::Card(c), None).0,
        Mc::Group(g) => duel::get_location_count_from_ex(d, tp, Some(tp), g.into(), duel::ExtraCard::Card(c), None).0,
    };
    count > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE)
    //     and s.filter1(chkc,e,tp,c:GetLevel(),c) end
    if let Some(chkc) = a.chkc() {
        let lv = card::get_level(d, c);
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && filter1(d, chkc, e, tp, lv, c);
    }
    // if chk==0 then return c:IsAbleToRemove()
    //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp,c:GetLevel(),c) end
    if a.chk == Some(0) {
        if !card::is_able_to_remove(d, c, None, None, None) {
            return false;
        }
        let lv = card::get_level(d, c);
        return duel::is_existing_target(d, Some(&|d, x| filter1(d, x, e, tp, lv, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp,c:GetLevel(),c)
    let lv = card::get_level(d, c);
    let mut g = duel::select_target(d, tp, Some(&|d, x| filter1(d, x, e, tp, lv, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc or not tc:IsRelateToEffect(e) then return end
    let Some(c) = c else { return };
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local lv=c:GetLevel()+tc:GetLevel()
    let lv = card::get_level(d, c) + card::get_level(d, tc);
    // local g=Group.FromCards(c,tc)
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(tc));
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)==2 then
    if duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv,g)
        let mc = Mc::Group(g.clone());
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| filter2(d, x, e, tp, lv, &mc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
