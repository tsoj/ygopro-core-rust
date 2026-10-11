//! Linkslayer @Ignister
use super::prelude::*;

pub const ID: u32 = 61168637;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_IGNISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If a Cyberse monster you control would be used as Link Material for an "@Ignister" Link Monster, this card in your hand can also be used as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetOperation(s.extracon)
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    // e1:SetValue(s.extraval)
    effect::set_value_fn(d, e1, lua_fn(extraval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 Spell/Trap on the field, or 2 if this card on the field was used as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(Cost.Discard())
    effect::set_cost(d, e2, Some(cost::discard(None, false, None, None, None)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The Lua table `s.flagmap[c]` (the flag effects this card registered as material), kept in the duel.
fn flag_key(c: CardId) -> u64 {
    0x10000 + c.0 as u64
}
fn flag_list(d: &Duel, c: CardId) -> Vec<EffectId> {
    match s_var(d, ID, flag_key(c)) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_flag_list(d: &mut Duel, c: CardId, list: Vec<EffectId>) {
    set_s_var(d, ID, flag_key(c), Value::Multi(list.into_iter().map(Value::Effect).collect()));
}
/// `function(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let _tp = a.p(2);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // return (sg+mg):Filter(s.extrafilter,nil,e:GetHandlerPlayer()):IsExists(Card.IsRace,1,og,RACE_CYBERSE)
    //     and sg:FilterCount(Card.HasFlagEffect,nil,id)<2
    let mut union = sg.clone();
    group::add_card(&mut union, &mg);
    let hp = effect::get_handler_player(d, e);
    let filtered = group::filter(d, &union, &|d, c| extrafilter(d, c, hp), NO_EX);
    let ex: Exclude = (&og).into();
    let exists = group::is_exists(d, &filtered, &|d, c| card::is_race(d, c, RACE_CYBERSE as u64), 1, ex);
    exists && group::filter_count(d, &sg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX) < 2
}
/// `s.extrafilter(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.c(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsSetCard(SET_IGNISTER) or Duel.HasFlagEffect(tp,id) then
        //     return Group.CreateGroup()
        if summon_type != SUMMON_TYPE_LINK as i64
            || !card::is_set_card(d, sc, &[SET_IGNISTER as u16])
            || duel::has_flag_effect(d, tp, ID, None)
        {
            return Value::Group(group::create_group(&[]));
        }
        // table.insert(s.flagmap[c],c:RegisterFlagEffect(id,0,0,1))
        let eff = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
        let mut list = flag_list(d, c);
        list.push(eff);
        set_flag_list(d, c, list);
        // return Group.FromCards(c)
        Value::Group(CardSet::from_card(c))
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK and #sg>0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 1);
        }
        Value::Nil
    } else if chk == 2 {
        // for _,eff in ipairs(s.flagmap[c]) do eff:Reset() end
        for eff in flag_list(d, c) {
            effect::reset(d, eff);
        }
        // s.flagmap[c]={}
        set_flag_list(d, c, Vec::new());
        Value::Nil
    } else {
        Value::Nil
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // e:SetLabel(c:IsPreviousLocation(LOCATION_ONFIELD) and 1 or 0)
    let label = if card::is_previous_location(d, c, LOCATION_ONFIELD) { 1 } else { 0 };
    effect::set_label(d, e, &[label]);
    // return c:IsLocation(LOCATION_GRAVE) and r==REASON_LINK
    card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_LINK
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1+e:GetLabel(),nil)
    let max = 1 + effect::get_label(d, e) as u16;
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, max, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
