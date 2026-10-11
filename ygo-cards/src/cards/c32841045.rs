//! Magical Musketeer Caspar
use super::prelude::*;

pub const ID: u32 = 32841045;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAGICAL_MUSKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // "Magical Musket" Spell/Traps can be activated from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e1, EFFECT_QP_ACT_IN_NTPHAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MAGICAL_MUSKET))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Magical Musket" card from the Deck to the hand
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // g:KeepAlive()
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Group(g));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetCondition(s.regcon)
    effect::set_condition(d, e4, Some(condition(regcon)));
    // e4:SetOperation(s.regop2)
    effect::set_operation(d, e4, Some(operation(regop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e5, EVENT_CUSTOM + ID);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetLabelObject(g)
    // (the group lives in e3's label object; e5 refers to e3, see TRANSLATING "shared group")
    effect::set_label_object(d, e5, Value::Effect(e3));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// The shared group of Caspar's effects: e3 holds it, the other effects point at e3.
fn label_group(d: &Duel, e: EffectId) -> CardSet {
    let first = effect::get_label_object(d, e).as_effect().nil();
    effect::get_label_object(d, first).as_group().cloned().nil()
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local flageff={c:GetFlagEffectLabel(1)}
    let flageff = card::get_flag_effect_label(d, c, 1);
    // local chainid=ev
    let chainid = a.ev as i64;
    // if flageff[1]==nil then return end
    if flageff.is_empty() {
        return;
    }
    // local g=e:GetLabelObject()
    // (this effect, e3, holds the group itself)
    let mut g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // for _,i in ipairs(flageff) do
    for i in flageff {
        // if chainid==i then
        if chainid == i {
            // if c:GetFlagEffect(2)==0 then
            if card::get_flag_effect(d, c, 2) == 0 {
                // g:Clear()
                g.clear();
                // store the cleared group now: the event below reads it
                effect::set_label_object(d, a.e, Value::Group(g.clone()));
                // c:RegisterFlagEffect(2,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1)
                card::register_flag_effect(d, c, 2, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
                // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,e,0,0,0,0)
                duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, 0, 0, 0);
            }
            // g:AddCard(re:GetHandler())
            let rc = effect::get_handler(d, a.re.nil()).nil();
            g.insert(rc);
            // store the group back (Lua groups are references)
            effect::set_label_object(d, a.e, Value::Group(g));
            // return
            return;
        }
    }
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local p,loc,seq=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_SEQUENCE)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE]);
    let p = info[0].int() as u8;
    let seq = info[2].int() as u32;
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and c:IsColumn(seq,p,LOCATION_SZONE)
    effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && card::is_column(d, c, seq, Some(p), Some(LOCATION_SZONE))
}
/// `s.regop2(e,tp,eg,ep,ev,re,r,rp)`
fn regop2(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(1,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1,ev)
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, 1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, a.ev as i64, 0);
}
/// `s.thfilter(c,codes)`
fn thfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsSetCard(SET_MAGICAL_MUSKET) and not c:IsCode(codes) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]) && !card::is_code(d, c, codes) && card::is_able_to_hand(d, c, None)
}
/// `s.chk(c,tp,e)`
fn chk(d: &mut Duel, c: CardId, tp: u8, _e: EffectId) -> bool {
    // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c,c:GetCode())
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, &[code])), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = label_group(d, a.e);
    // if chk==0 then return g:IsExists(s.chk,1,nil,tp,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &g, &|d, c| chk(d, c, a.tp, a.e), 1, NO_EX);
    }
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // e:SetLabel(g:Select(tp,1,1,nil):GetFirst():GetCode())
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let first = group::get_first(d, &sel).nil();
        let code = card::get_code(d, first)[0] as i64;
        effect::set_label(d, a.e, &[code]);
    } else {
        // e:SetLabel(g:GetFirst():GetCode())
        let first = group::get_first(d, &g).nil();
        let code = card::get_code(d, first)[0] as i64;
        effect::set_label(d, a.e, &[code]);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local codes={e:GetLabel()}
    let codes = [effect::get_label(d, a.e) as u32];
    // if not #codes==0 then return end
    // (Lua: (not #codes)==0 is always false, so the script never returns here)
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,e:GetLabelObject(),codes)
    let exg = label_group(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, &codes)), a.tp, LOCATION_DECK, 0, 1, 1, false, (&exg).into()).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
