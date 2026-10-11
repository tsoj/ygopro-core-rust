//! Deus X-Krawler
use super::prelude::*;

pub const ID: u32 = 62587693;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of an opponent's card or effect that targets this face-down card, and if you do, destroy it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetCost(Cost.SelfChangePosition(POS_FACEUP_DEFENSE))
    effect::set_cost(d, e1, Some(cost::self_change_position(POS_FACEUP_DEFENSE as u32)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // After this card was flipped face-up, while it is in the Monster Zone, negate all monster effects activated on your opponent's field
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2a:SetCode(EVENT_FLIP)
    effect::set_code(d, e2a, EVENT_FLIP);
    // e2a:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2)) end)
    effect::set_operation(d, e2a, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    })));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2b, EVENT_CHAIN_SOLVING);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():HasFlagEffect(id) and re:IsMonsterEffect() and rp==1-tp and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE end)
    effect::set_condition(d, e2b, Some(condition(e2b_cond)));
    // e2b:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e2b, Some(operation(|d, a| {
        duel::negate_effect(d, a.ev as u8);
    })));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Add 1 Level 9 monster with a different original Type and Attribute than this card from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_ONFIELD) && (a.r & (REASON_BATTLE | REASON_EFFECT)) > 0
    })));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (rp==1-tp and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and c:IsFacedown() and Duel.IsChainNegatable(ev)) then return false end
    if !(a.rp == 1 - a.tp
        && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0)
        && card::is_facedown(d, c)
        && duel::is_chain_negatable(d, a.ev as u8))
    {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // return tg and tg:IsContains(c)
    match tg[0].as_group() {
        Some(tg) => group::is_contains(tg, c),
        None => false,
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.thfilter(c,race,attribute)`
fn thfilter(d: &mut Duel, c: CardId, race: u64, attribute: u32) -> bool {
    // return c:IsLevel(9) and c:IsAbleToHand() and not c:IsOriginalRace(race) and not c:IsOriginalAttribute(attribute)
    card::is_level(d, c, &[9]) && card::is_able_to_hand(d, c, None) && !card::is_original_race(d, c, race) && !card::is_original_attribute(d, c, attribute)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetOriginalRace(),c:GetOriginalAttribute())
        let race = card::get_original_race(d, c);
        let attr = card::get_original_attribute(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, race, attr)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,c:GetOriginalRace(),c:GetOriginalAttribute())
    let race = card::get_original_race(d, c);
    let attr = card::get_original_attribute(d, c);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| thfilter(d, cc, race, attr)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `e:GetHandler():IsRelateToEffect` ... the condition of e2b (`function(e,tp,eg,ep,ev,re,r,rp)`)
fn e2b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // e:GetHandler():HasFlagEffect(id) and re:IsMonsterEffect() and rp==1-tp and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE
    let h = effect::get_handler(d, a.e).nil();
    if !card::has_flag_effect(d, h, ID, None) {
        return false;
    }
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    if a.rp != 1 - a.tp {
        return false;
    }
    duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() == LOCATION_MZONE as i64
}
