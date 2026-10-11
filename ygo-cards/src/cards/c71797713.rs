//! Elementsaber Lapauila Mana
use super::prelude::*;

pub const ID: u32 = 71797713;

const CARD_ELEMENTAL_PLACE: u32 = 61557074;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTSABER, SET_ELEMENTAL_LORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register effect when it is summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToGraveAsCost() and (c:IsSetCard(SET_ELEMENTSABER) or c:IsLocation(LOCATION_HAND))
    card::is_monster(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) || card::is_location(d, c, LOCATION_HAND))
}
// function s.regfilter(c,attr)
fn regfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsSetCard(SET_ELEMENTSABER) and c:GetOriginalAttribute()&attr~=0
    card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) && (card::get_original_attribute(d, c) & attr) != 0
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local fg=Group.CreateGroup()
    let mut fg = CardSet::new();
    // for i,pe in ipairs({Duel.IsPlayerAffectedByEffect(tp,CARD_ELEMENTAL_PLACE)}) do
    //     fg:AddCard(pe:GetHandler())
    // end
    if let Some(pe) = duel::is_player_affected_by_effect(d, a.tp, CARD_ELEMENTAL_PLACE) {
        let h = effect::get_handler(d, pe).nil();
        group::add_card(&mut fg, &CardSet::from_card(h));
    }
    // local loc=LOCATION_HAND
    let mut loc = LOCATION_HAND;
    // if #fg>0 then loc=LOCATION_HAND|LOCATION_DECK end
    if !fg.is_empty() {
        loc = LOCATION_HAND | LOCATION_DECK;
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,loc,0,2,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, loc, 0, 2, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,loc,0,2,2,e:GetHandler())
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, loc, 0, 2, 2, false, h.into()).nil();
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        // local fc=nil
        // if #fg==1 then fc=fg:GetFirst() else fc=fg:Select(tp,1,1,nil) end
        let fc = if fg.len() == 1 {
            group::get_first(d, &fg).nil()
        } else {
            // fc is then a group, and fc:GetCode() on a group is a Lua error
            let _sel = group::select(d, &fg, a.tp, 1, 1, false, NO_EX).nil();
            crate::lua_error("attempt to call a nil value")
        };
        // Duel.Hint(HINT_CARD,0,fc:GetCode())
        let code = card::get_code(d, fc)[0];
        duel::hint(d, HINT_CARD as u8, 0, code as u64);
        // fc:RegisterFlagEffect(CARD_ELEMENTAL_PLACE,RESETS_STANDARD_PHASE_END,0,0)
        card::register_flag_effect(d, fc, CARD_ELEMENTAL_PLACE, RESETS_STANDARD_PHASE_END, 0, 0, 0, 0);
    }
    // local flag=0
    let mut flag: i64 = 0;
    // if g:IsExists(s.regfilter,1,nil,ATTRIBUTE_EARTH|ATTRIBUTE_WIND) then flag=flag|0x1 end
    if group::is_exists(d, &g, &|d, c| regfilter(d, c, ATTRIBUTE_EARTH | ATTRIBUTE_WIND), 1, NO_EX) {
        flag |= 0x1;
    }
    // if g:IsExists(s.regfilter,1,nil,ATTRIBUTE_WATER|ATTRIBUTE_FIRE) then flag=flag|0x2 end
    if group::is_exists(d, &g, &|d, c| regfilter(d, c, ATTRIBUTE_WATER | ATTRIBUTE_FIRE), 1, NO_EX) {
        flag |= 0x2;
    }
    // if g:IsExists(s.regfilter,1,nil,ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) then flag=flag|0x4 end
    if group::is_exists(d, &g, &|d, c| regfilter(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK), 1, NO_EX) {
        flag |= 0x4;
    }
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // e:SetLabel(flag)
    effect::set_label(d, a.e, &[flag]);
    true
}
// function s.costfilter for Duel.SelectMatchingCard / IsExistingMatchingCard (see above)
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.regcon(e,tp,eg,ep,ev,re,r,rp)
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1 and e:GetLabelObject():GetLabel()~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1 && {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::get_label(d, lo) != 0
    }
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local flag=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let flag = effect::get_label(d, lo);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e0, EFFECT_FLAG_CLIENT_HINT, 0);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e0, LOCATION_MZONE, 0);
    // e0:SetTarget(s.immtg)
    effect::set_target(d, e0, Some(effect_card_fn(immtg)));
    // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
    // if flag&0x1~=0 then
    if (flag & 0x1) != 0 {
        // local e1=e0:Clone()
        let e1 = effect::clone(d, e0);
        // e1:SetDescription(3000)
        effect::set_description(d, e1, 3000);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if flag&0x2~=0 then
    if (flag & 0x2) != 0 {
        // local e2=e0:Clone()
        let e2 = effect::clone(d, e0);
        // e2:SetDescription(3001)
        effect::set_description(d, e2, 3001);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // if flag&0x4~=0 then
    if (flag & 0x4) != 0 {
        // local e3=e0:Clone()
        let e3 = effect::clone(d, e0);
        // e3:SetDescription(3061)
        effect::set_description(d, e3, 3061);
        // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e3:SetValue(aux.tgoval)
        effect::set_value_fn(d, e3, aux::tgoval());
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
// function s.immtg(e,c)
fn immtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELEMENTSABER) or c:IsSetCard(SET_ELEMENTAL_LORD)
    card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) || card::is_set_card(d, c, &[SET_ELEMENTAL_LORD as u16])
}
