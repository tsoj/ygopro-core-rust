//! Gearfried the Steel Knight
use super::prelude::*;

pub const ID: u32 = 100459023;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of a monster(s) on the field being destroyed by battle or card effect
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
        effect::set_operation(d, ge1, Some(operation(ge1_op)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // aux.AddEREquipLimit(c,nil,function(ec,_,tp) return s.eqfilter(ec,tp) end,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(er_equipval), lua_fn(er_equipop), e1, None, None, None);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_monster(d, c) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
/// `function(ec,_,tp) return s.eqfilter(ec,tp) end` (the equip limit)
fn er_equipval(d: &mut Duel, a: Args) -> bool {
    let ec = a.c(0);
    let tp = a.p(2);
    eqfilter(d, ec, tp)
}
/// `s.equipop` as the equip operation of aux.AddEREquipLimit
fn er_equipop(d: &mut Duel, a: Args) -> Value {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
    Value::Nil
}
/// `function(ge1 ...)`: the flag for monsters destroyed by battle or card effect
fn ge1_op(d: &mut Duel, a: &EffArgs) {
    // for ec in eg:Iter() do
    let eg = a.eg();
    for ec in group::iter(d, &eg) {
        // local prev_ctlr=ec:GetPreviousControler()
        let prev_ctlr = card::get_previous_controler(d, ec);
        // if ec:IsPreviousLocation(LOCATION_MZONE) and ec:IsReason(REASON_BATTLE|REASON_EFFECT)
        //     and not Duel.HasFlagEffect(prev_ctlr,id) then
        if card::is_previous_location(d, ec, LOCATION_MZONE)
            && card::is_reason(d, ec, REASON_BATTLE | REASON_EFFECT)
            && !duel::has_flag_effect(d, prev_ctlr, ID, None)
        {
            // Duel.RegisterFlagEffect(prev_ctlr,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, prev_ctlr, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if ec then
        if let Some(ec) = group::get_first(d, &sel) {
            // s.equipop(c,e,tp,ec)
            equipop(d, c, a.e, tp, ec);
        }
    }
}
/// `s.equipop(c,e,tp,ec)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,ec,id) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, ec, Some(ID), None) {
        return;
    }
    // Equip 1 monster from your GY to it as an Equip Spell that gives it 800 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // ec:RegisterEffect(e1)
    card::register_effect(d, ec, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetEquipGroup()
    let c = effect::get_handler(d, a.e).nil();
    let g = card::get_equip_group(d, c);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.propfilter(c,attr,race,lv)`
fn propfilter(d: &mut Duel, c: CardId, attr: u32, race: u64, lv: u32) -> bool {
    // return c:IsAttribute(attr) and c:IsRace(race) and not c:IsLevel(lv)
    card::is_attribute(d, c, attr) && card::is_race(d, c, race) && !card::is_level(d, c, &[lv])
}
/// `s.thfilter(c,mg)`
fn thfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsMonster() and mg:IsExists(s.propfilter,1,nil,c:GetAttribute(),c:GetRace(),c:GetLevel())
    if !card::is_monster(d, c) {
        return false;
    }
    let attr = card::get_attribute(d, c);
    let race = card::get_race(d, c);
    let lv = card::get_level(d, c);
    group::is_exists(d, mg, &|d, x| propfilter(d, x, attr, race, lv), 1, NO_EX)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=c:GetEquipGroup()
    let g = card::get_equip_group(d, c);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local mg=Duel.GetOperatedGroup():Filter(aux.AND(Card.IsMonsterCard,Card.HasLevel),nil)
        let operated = duel::get_operated_group(d);
        let mg = group::filter(d, &operated, &|d, x| card::is_monster_card(d, x) && card::has_level(d, x), NO_EX);
        // if #mg>0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,mg)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !mg.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, &mg)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,mg)
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, &mg)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - tp, sg);
            }
        }
    }
}
