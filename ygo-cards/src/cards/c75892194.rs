//! Ancient Gear Dragon
use super::prelude::*;

pub const ID: u32 = 75892194;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_GEAR_GOLEM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:AddCannotBeSpecialSummoned()
    card::add_cannot_be_special_summoned(d, c);
    // Normal Summon this card without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.nscon)
    effect::set_condition(d, e1, Some(lua_fn(nscon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate an opponent's activated Spell/Trap Card or effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsSpellTrapEffect() and Duel.IsChainDisablable(ev) end)
    effect::set_condition(d, e2, Some(condition(|d, a| a.ep == 1 - a.tp && effect::is_spell_trap_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8))));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::negate_effect(d, a.ev as u8);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // "Double Snare" check
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}

/// `s.nsconfilter(c)`: `c:IsFaceup() and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_MACHINE)`
fn nsconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_MACHINE)
}

/// `s.nscon(e)`: the Normal Summon condition; the core passes `(e, c)`
fn nscon(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return false end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return false;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // return #g==0 or g:FilterCount(s.nsconfilter,nil)==#g
    g.is_empty() || group::filter_count(d, &g, &nsconfilter, NO_EX) == g.len()
}

/// `s.discostfilter(c)`
fn discostfilter(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsAbleToGraveAsCost() then return false end
    if !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then return c:IsCode(CARD_ANCIENT_GEAR_GOLEM)
    if card::is_location(d, c, LOCATION_DECK) {
        return card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]);
    }
    // else return c:IsRace(RACE_MACHINE) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) end
    card::is_race(d, c, RACE_MACHINE) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}

/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let loc = LOCATION_HAND | LOCATION_MZONE | LOCATION_DECK;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.discostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discostfilter), tp, loc, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.discostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&discostfilter), tp, loc, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}

/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
