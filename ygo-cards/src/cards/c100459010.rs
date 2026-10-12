//! Silent Magician, the Silent Fighter
//! 沈黙の闘者－サイレント・マジシャン
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100459010;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned (from your hand) by Tributing 1 Level 7 or lower monster (Spellcaster or Warrior). You can only Special Summon "Silent Magician, the Silent Fighter" once per turn this way
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(cost_target(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(operation(spop)));
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // The first time this card would be destroyed by battle or card effect each turn, it is not destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetValue(function(e,re,r) return r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_value_fn(d, e1, lua_fn(|_d, a| (a.int(2) as u32 & (REASON_BATTLE | REASON_EFFECT)) > 0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Twice per turn, each time your opponent draws a card(s): You can draw 1 card, or if your opponent drew by their own card effect, you draw the same number of cards instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DRAW)
    effect::set_code(d, e2, EVENT_DRAW);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(2)
    effect::set_count_limit(d, e2, 2, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While you have 6 or more cards in your hand, negate all your opponent's activated Spell Cards and effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>=6 end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) >= 6)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(e3_operation)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(7) and c:IsRace(RACE_SPELLCASTER|RACE_WARRIOR)
    card::is_level_below(d, c, 7) && card::is_race(d, c, RACE_SPELLCASTER | RACE_WARRIOR)
}

/// `s.spcon(e,c)`: `function(e,c) if c==nil then return true end ... end`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.spcostfilter,1,false,1,true,c,tp,nil,false,nil)
    duel::check_release_group(
        d,
        tp,
        Some(&spcostfilter),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, cancelable: false },
        NO_EX,
        false,
    )
}

/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`: the label object of the summon is the group released
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.SelectReleaseGroup(tp,s.spcostfilter,1,1,false,true,true,c,tp,nil,false,nil)
    // (the card `c` is the 10th argument, which EffArgs keeps as `chkc`)
    let c = a.chkc();
    let g = duel::select_release_group(
        d,
        a.tp,
        Some(&spcostfilter),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: c, toplayer: Some(a.tp), zone: None, use_oppo: false, max: None },
        NO_EX,
        false,
    );
    // if g then
    match g {
        Some(g) => {
            // e:SetLabelObject(g)
            effect::set_label_object(d, a.e, Value::Group(g));
            // return true
            true
        }
        // return false
        None => false,
    }
}

/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = match effect::get_label_object(d, a.e) {
        Value::Group(g) => Some(g),
        _ => None,
    };
    // if g then
    if let Some(g) = g {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    }
}

/// `s.e2_target`: `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local opp_eff_chk=rp==1-tp and r&REASON_EFFECT>0
    let opp_eff_chk = a.rp == 1 - a.tp && (a.r & REASON_EFFECT) > 0;
    // local draw_count=opp_eff_chk and #eg or 1
    let draw_count: u32 = if opp_eff_chk { a.eg.clone().nil().len() as u32 } else { 1 };
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,draw_count) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, draw_count);
    }
    // e:GetChainData().draw_count=draw_count
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(draw_count as i64));
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,draw_count)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, draw_count as i32);
    true
}

/// `s.e2_operation`: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    // local draw_count=e:GetChainData().draw_count
    let draw_count = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
    // Duel.Draw(tp,draw_count,REASON_EFFECT)
    duel::draw(d, a.tp, draw_count, REASON_EFFECT);
}

/// `s.e3_operation`: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e3_operation(d: &mut Duel, a: &EffArgs) {
    // if rp==1-tp and re:IsSpellEffect() then
    if a.rp == 1 - a.tp && effect::is_spell_effect(d, a.re.nil()) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
