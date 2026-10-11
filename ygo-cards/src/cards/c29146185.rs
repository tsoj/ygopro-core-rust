//! World of Prophecy
use super::prelude::*;

pub const ID: u32 = 29146185;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // ss success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetCost(s.retcost)
    effect::set_cost(d, e1, Some(cost_target(retcost)));
    // e1:SetTarget(s.rettg)
    effect::set_target(d, e1, Some(cost_target(rettg)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re then return false end
    let Some(re) = a.re else {
        return false;
    };
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsRace(RACE_SPELLCASTER) or (rc:IsSetCard(SET_SPELLBOOK) and rc:IsSpell())
    card::is_race(d, rc, RACE_SPELLCASTER) || (card::is_set_card(d, rc, &[SET_SPELLBOOK as u16]) && card::is_spell(d, rc))
}
/// `s.retcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn retcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==1 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON as u32]).first().copied().unwrap_or(0) == 1;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,2,2,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).unwrap_or_default();
    // if Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // if c:IsFaceup() and c:IsRelateToEffect(e) then
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) {
            // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,0,0)
            duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r as u32, a.rp, 0, 0);
        }
    }
}
/// `s.cffilter(c)`
fn cffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsSpell() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c) && !card::is_public(d, c)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cffilter,tp,LOCATION_HAND,0,nil)
    let mut g = duel::get_matching_group(d, Some(&cffilter), tp, LOCATION_HAND, 0, NO_EX);
    // if chk==0 then return g:GetClassCount(Card.GetCode)>=4 end
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 4;
    }
    // local cg=Group.CreateGroup()
    let mut cg = CardSet::new();
    // for i=1,4 do
    for _ in 1..=4 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,sg:GetFirst():GetCode())
        let first = group::get_first(d, &sg).nil();
        let codes = card::get_code(d, first);
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
        // cg:Merge(sg)
        group::add_card(&mut cg, &sg);
    }
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - tp, cg);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()  (the Lua calls e:GetHandler() below)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
