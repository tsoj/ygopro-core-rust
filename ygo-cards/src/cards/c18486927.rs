//! Ancient Gear Gadget
//! 古代の歯車機械
use super::prelude::*;

pub const ID: u32 = 18486927;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GADGET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // declare card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetTarget(s.dectg)
    effect::set_target(d, e1, Some(cost_target(dectg)));
    // e1:SetOperation(s.decop)
    effect::set_operation(d, e1, Some(operation(decop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // name change
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.nametg)
    effect::set_target(d, e3, Some(cost_target(nametg)));
    // e3:SetOperation(s.nameop)
    effect::set_operation(d, e3, Some(operation(nameop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.dectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    // e:SetLabel(Duel.SelectOption(tp,70,71,72))
    let opt = duel::select_option(d, a.tp, true, &[70, 71, 72]);
    effect::set_label(d, a.e, &[opt as i64]);
    true
}
/// `s.decop(e,tp,eg,ep,ev,re,r,rp)`
fn decop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local opt=e:GetLabel()
    let opt = effect::get_labels(d, a.e)[0];
    // local ct=nil
    // if opt==0 then ct=TYPE_MONSTER elseif opt==1 then ct=TYPE_SPELL else ct=TYPE_TRAP end
    let ct = if opt == 0 {
        TYPE_MONSTER
    } else if opt == 1 {
        TYPE_SPELL
    } else {
        TYPE_TRAP
    };
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetLabel(ct)
    effect::set_label(d, e1, &[ct as i64]);
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(condition(actcon)));
    // e1:SetValue(s.actlimit)
    effect::set_value_fn(d, e1, lua_fn(actlimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.actlimit(e,re,tp)`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    // local ct=e:GetLabel()
    let ct = effect::get_labels(d, e)[0];
    // return re:IsActiveType(ct) and (ct==TYPE_MONSTER or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    effect::is_active_type(d, re, ct as u32) && (ct == TYPE_MONSTER as i64 || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return tc and tc:IsControler(tp)
    match duel::get_attacker(d) {
        Some(tc) => card::is_controler(d, tc, tp),
        None => false,
    }
}
/// `s.nametg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nametg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local code=e:GetHandler():GetCode()
    let c = effect::get_handler(d, a.e).nil();
    let code = card::get_code(d, c)[0] as u64;
    // s.announce_filter={SET_GADGET,OPCODE_ISSETCARD,code,OPCODE_ISCODE,OPCODE_NOT,OPCODE_AND,TYPE_MONSTER,OPCODE_ISTYPE,OPCODE_AND}
    let announce_filter = [
        SET_GADGET as u64,
        OPCODE_ISSETCARD as u64,
        code,
        OPCODE_ISCODE as u64,
        OPCODE_NOT as u64,
        OPCODE_AND as u64,
        TYPE_MONSTER as u64,
        OPCODE_ISTYPE as u64,
        OPCODE_AND as u64,
    ];
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac = duel::announce_card(d, a.tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.nameop(e,tp,eg,ep,ev,re,r,rp)`
fn nameop(d: &mut Duel, a: &EffArgs) {
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
    let ac = info[0].int();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetValue(ac)
        effect::set_value(d, e1, ac);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
