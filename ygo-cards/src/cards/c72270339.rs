//! Diabellstar the Black Witch
use super::prelude::*;

pub const ID: u32 = 72270339;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SINFUL_SPOILS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand) by sending 1 card from your hand or field to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spproccon)
    effect::set_condition(d, e1, Some(lua_fn(spproccon)));
    // e1:SetTarget(s.spproctg)
    effect::set_target(d, e1, Some(lua_fn(spproctg)));
    // e1:SetOperation(s.spprocop)
    effect::set_operation(d, e1, Some(lua_fn(spprocop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Sinful Spoils" Spell/Trap directly from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Send 1 card from your hand or field to the GY, and if you do, Special Summon this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    // e4:SetCondition(s.spgycon)
    effect::set_condition(d, e4, Some(condition(spgycon)));
    // e4:SetTarget(s.spgytg)
    effect::set_target(d, e4, Some(cost_target(spgytg)));
    // e4:SetOperation(s.spgyop)
    effect::set_operation(d, e4, Some(operation(spgyop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgfilter(c,tp,bool)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, is_grave: bool) -> bool {
    // local tg_check=nil
    // if bool then tg_check=c:IsAbleToGrave() else tg_check=c:IsAbleToGraveAsCost() end
    let tg_check = if is_grave { card::is_able_to_grave(d, c, None) } else { card::is_able_to_grave_as_cost(d, c) };
    // return tg_check and Duel.GetMZoneCount(tp,c)>0
    tg_check && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.spproccon(e,c)`
fn spproccon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,c,tp,false)
    let rg = duel::get_matching_group(d, Some(&|d, x| tgfilter(d, x, tp, false)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.into());
    // return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,1,nil,0)
    !rg.is_empty() && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.spproctg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn spproctg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(9);
    // local rg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,c,tp,false)
    let rg = duel::get_matching_group(d, Some(&|d, x| tgfilter(d, x, tp, false)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.map_or(NO_EX, |c| c.into()));
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()  (nothing to do: groups are values)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spprocop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spprocop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()  (nothing to do: groups are values)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SINFUL_SPOILS) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_SINFUL_SPOILS as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SSet(tp,g) end
    if !g.is_empty() {
        duel::s_set(d, a.tp, g, None, true);
    }
}
/// `s.spgycon(e,tp,eg,ep,ev,re,r,rp)`
fn spgycon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) and c:IsPreviousControler(tp) and Duel.IsTurnPlayer(1-tp)
    card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD) && card::is_previous_controler(d, c, a.tp) && duel::is_turn_player(d, 1 - a.tp)
}
/// `s.spgytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spgytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil,tp,true) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, tp, true)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spgyop(e,tp,eg,ep,ev,re,r,rp)`
fn spgyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil,tp,true):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| tgfilter(d, x, tp, true)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE)
    //   and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0
            && card::is_location(d, tc, LOCATION_GRAVE)
            && card::is_relate_to_effect(d, c, a.e)
        {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
