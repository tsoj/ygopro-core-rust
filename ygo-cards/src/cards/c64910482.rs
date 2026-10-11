//! T.G. Cyber Magician
use super::prelude::*;

pub const ID: u32 = 64910482;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to grave
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // hand synchro
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_HAND_SYNCHRO)
    effect::set_code(d, e3, EFFECT_HAND_SYNCHRO);
    // e3:SetLabel(id)
    effect::set_label(d, e3, &[ID as i64]);
    // e3:SetValue(s.synval)
    effect::set_value_fn(d, e3, lua_fn(synval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.synval(e,c,sc)`
fn synval(d: &mut Duel, a: Args) -> bool {
    let (e, c) = (a.e(0), a.c(1));
    let sc = a.card(2).nil();
    // if sc:IsSetCard(SET_TG) and --c:IsNotTuner()
    //     (not c:IsType(TYPE_TUNER) or c:IsHasEffect(EFFECT_NONTUNER)) and c:IsSetCard(SET_TG) and c:IsLocation(LOCATION_HAND) then
    if card::is_set_card(d, sc, &[SET_TG as u16])
        && (!card::is_type(d, c, TYPE_TUNER) || !card::is_has_effect(d, c, EFFECT_NONTUNER, None).is_empty())
        && card::is_set_card(d, c, &[SET_TG as u16])
        && card::is_location(d, c, LOCATION_HAND)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK)
        effect::set_code(d, e1, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
        // e1:SetLabel(id)
        effect::set_label(d, e1, &[ID as i64]);
        // e1:SetTarget(s.synchktg)
        effect::set_target(d, e1, Some(lua_fn(synchktg)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // return true
        return true;
    }
    // else return false end
    false
}
/// `s.chk2(c)`
fn chk2(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsHasEffect(EFFECT_HAND_SYNCHRO) or c:IsHasEffect(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK) then return false end
    if card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO, None).is_empty() || !card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty() {
        return false;
    }
    // local te={c:GetCardEffect(EFFECT_HAND_SYNCHRO)}
    let te = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO);
    // for i=1,#te do
    for e in te {
        // local e=te[i]
        // if e:GetLabel()==id then return true end
        if effect::get_label(d, e) == ID as i64 {
            return true;
        }
    }
    // return false
    false
}
/// `s.synchktg(e,c,sg,tg,ntg,tsg,ntsg)`
fn synchktg(d: &mut Duel, a: Args) -> Value {
    // if c then
    if let Some(c) = a.card(1) {
        let sg = a.g(2);
        let tg = a.g(3);
        let ntg = a.g(4);
        // local res=tg:IsExists(s.chk2,1,c) or ntg:IsExists(s.chk2,1,c) or sg:IsExists(s.chk2,1,c)
        let res = group::is_exists(d, &tg, &chk2, 1, c.into()) || group::is_exists(d, &ntg, &chk2, 1, c.into()) || group::is_exists(d, &sg, &chk2, 1, c.into());
        // return res,Group.CreateGroup(),Group.CreateGroup()
        return Value::Multi(vec![Value::Bool(res), Value::Group(CardSet::new()), Value::Group(CardSet::new())]);
    }
    // else return true end
    Value::Bool(true)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsReason(REASON_DESTROY) then
    if card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_DESTROY) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetRange(LOCATION_GRAVE)
        effect::set_range(d, e1, LOCATION_GRAVE);
        // e1:SetTarget(s.thtg)
        effect::set_target(d, e1, Some(cost_target(thtg)));
        // e1:SetOperation(s.thop)
        effect::set_operation(d, e1, Some(operation(thop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TG) and c:GetCode()~=id and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TG as u16]) && card::get_code(d, c).first().copied() != Some(ID) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
