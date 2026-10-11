//! Awakening of the Possessed - Greater Inari Fire
//! 憑依覚醒－大稲荷火
use super::prelude::*;

pub const ID: u32 = 92652813;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPIRITUAL_FIRE_ART, SET_POSSESSED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Sumon Procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_DECK)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_DECK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.dmcon)
    effect::set_condition(d, e2, Some(condition(dmcon)));
    // e2:SetTarget(s.dmtg)
    effect::set_target(d, e2, Some(cost_target(dmtg)));
    // e2:SetOperation(s.dmop)
    effect::set_operation(d, e2, Some(operation(dmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter1(c)`
fn spfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spfilter2(c)`
fn spfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsLevelBelow(4) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_level_below(d, c, 4) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg,tp)
    let res = aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0 && group::is_exists(d, sg, &|d, c| chk(d, c, sg, tp), 1, NO_EX);
    (res, false)
}
/// `s.chk(c,sg,tp)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet, _tp: u8) -> bool {
    // return s.spfilter1(c,tp) and sg:IsExists(s.spfilter2,1,c)
    spfilter1(d, c) && group::is_exists(d, sg, &spfilter2, 1, c.into())
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=g1:Clone()
    let mut g = g1.clone();
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #g1>0 and #g2>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !g1.is_empty()
        && !g2.is_empty()
        && matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e).nil();
    // local g1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE,0,nil)
    let mut g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE, 0, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local g=aux.SelectUnselectGroup(g1,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &g1, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // if e:GetHandler():IsLocation(LOCATION_DECK) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_DECK) {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
    }
}
/// `s.dmcon(e,tp,eg,ep,ev,re,r,rp)`
fn dmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.dmgfilter(c)`
fn dmgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetBaseAttack()>0
    card::is_faceup(d, c) && card::get_base_attack(d, c) > 0
}
/// `s.dmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.dmgfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&dmgfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.dmop(e,tp,eg,ep,ev,re,r,rp)`
fn dmop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_OPPO as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.dmgfilter,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&dmgfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Damage(p,tc:GetBaseAttack(),REASON_EFFECT)
        let atk = card::get_base_attack(d, tc);
        duel::damage(d, p, atk as i64, REASON_EFFECT, false, None);
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and (c:IsSetCard(SET_SPIRITUAL_FIRE_ART) or (c:IsSetCard(SET_POSSESSED) and c:IsSpellTrap()))
    card::is_able_to_hand(d, c, None)
        && (card::is_set_card(d, c, &[SET_SPIRITUAL_FIRE_ART as u16]) || (card::is_set_card(d, c, &[SET_POSSESSED as u16]) && card::is_spell_trap(d, c)))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
